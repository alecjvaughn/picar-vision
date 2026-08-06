use image::{imageops::FilterType, GenericImageView};
use ndarray::{s, Array, IxDyn};
use ort::session::builder::GraphOptimizationLevel;
use ort::session::Session;
use ort::value::Tensor;
use std::sync::Arc;
use tokio::sync::Mutex;
use base64::{Engine as _, engine::general_purpose::STANDARD};

pub struct VisionState {
    pub session: Mutex<Option<Arc<Mutex<Session>>>>,
    pub init_error: Mutex<Option<String>>,
    pub target_class: Mutex<String>,
    pub autonomous_mode: Mutex<bool>,
}

const MODEL_BYTES: &[u8] = include_bytes!("../assets/yolov8n.onnx");

pub fn init_vision() -> Result<Session, Box<dyn std::error::Error>> {
    let _ = ort::init().with_name("YOLOv8").commit();

    let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_intra_threads(4)?
        .commit_from_memory(MODEL_BYTES)?;

    Ok(session)
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub confidence: f32,
    pub class_id: usize,
    pub label: String,
}

pub fn process_frame(
    session: &mut Session,
    base64_jpg: &str,
    conf_threshold: f32,
    iou_threshold: f32,
) -> Result<(Vec<BoundingBox>, String, f32), Box<dyn std::error::Error>> {
    let img_bytes = STANDARD.decode(base64_jpg)?;
    let img = image::load_from_memory(&img_bytes)?;
    let (orig_width, orig_height) = img.dimensions();

    // YOLOv8 expects 640x640 input
    let resized = img.resize_exact(640, 640, FilterType::CatmullRom);
    let rgb = resized.to_rgb8();

    // Convert to NCHW f32 tensor (1, 3, 640, 640)
    let mut input_array = Array::zeros(IxDyn(&[1, 3, 640, 640]));
    for (x, y, pixel) in rgb.enumerate_pixels() {
        let x = x as usize;
        let y = y as usize;
        input_array[[0, 0, y, x]] = (pixel[0] as f32) / 255.0;
        input_array[[0, 1, y, x]] = (pixel[1] as f32) / 255.0;
        input_array[[0, 2, y, x]] = (pixel[2] as f32) / 255.0;
    }

    let input_value = Tensor::from_array(input_array)?;
    let outputs = session.run(ort::inputs![input_value])?;
    
    // Output shape for YOLOv8n: [1, 84, 8400]
    let (shape, slice) = outputs[0].try_extract_tensor::<f32>()?;
    let num_anchors = shape[2] as usize;
    let num_features = shape[1] as usize;
    
    let mut boxes = Vec::new();
    
    let x_scale = orig_width as f32 / 640.0;
    let y_scale = orig_height as f32 / 640.0;

    let mut absolute_max_conf = 0.0;

    for i in 0..num_anchors {
        // Data is flattened in [1, 84, 8400]. 
        // For anchor `i`, feature `f` is at index: f * 8400 + i
        let cx = slice[0 * num_anchors + i];
        let cy = slice[1 * num_anchors + i];
        let w = slice[2 * num_anchors + i];
        let h = slice[3 * num_anchors + i];

        let mut max_conf = 0.0;
        let mut class_id = 0;
        for c in 0..80 {
            let conf = slice[(4 + c) * num_anchors + i];
            if conf > max_conf {
                max_conf = conf;
                class_id = c;
            }
        }
        
        if max_conf > absolute_max_conf {
            absolute_max_conf = max_conf;
        }

        if max_conf > conf_threshold {
            // Convert center (cx, cy) to top-left (x, y) and scale back to original image
            let width = w * x_scale;
            let height = h * y_scale;
            let x = (cx * x_scale) - (width / 2.0);
            let y = (cy * y_scale) - (height / 2.0);

            boxes.push(BoundingBox {
                x,
                y,
                width,
                height,
                confidence: max_conf,
                class_id,
                label: get_class_name(class_id),
            });
        }
    }

    // NMS
    let mut nms_boxes = Vec::new();
    boxes.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
    
    let mut active = vec![true; boxes.len()];
    for i in 0..boxes.len() {
        if !active[i] { continue; }
        nms_boxes.push(boxes[i].clone());
        for j in (i + 1)..boxes.len() {
            if active[j] && iou(&boxes[i], &boxes[j]) > iou_threshold {
                active[j] = false;
            }
        }
    }

    Ok((nms_boxes, format!("{:?}", shape), absolute_max_conf))
}

fn iou(a: &BoundingBox, b: &BoundingBox) -> f32 {
    let x1 = f32::max(a.x, b.x);
    let y1 = f32::max(a.y, b.y);
    let x2 = f32::min(a.x + a.width, b.x + b.width);
    let y2 = f32::min(a.y + a.height, b.y + b.height);

    if x2 < x1 || y2 < y1 { return 0.0; }
    
    let intersection = (x2 - x1) * (y2 - y1);
    let area_a = a.width * a.height;
    let area_b = b.width * b.height;
    
    intersection / (area_a + area_b - intersection)
}

fn get_class_name(id: usize) -> String {
    let classes = [
        "person", "bicycle", "car", "motorcycle", "airplane", "bus", "train", "truck", "boat", "traffic light",
        "fire hydrant", "stop sign", "parking meter", "bench", "bird", "cat", "dog", "horse", "sheep", "cow",
        "elephant", "bear", "zebra", "giraffe", "backpack", "umbrella", "handbag", "tie", "suitcase", "frisbee",
        "skis", "snowboard", "sports ball", "kite", "baseball bat", "baseball glove", "skateboard", "surfboard", "tennis racket", "bottle",
        "wine glass", "cup", "fork", "knife", "spoon", "bowl", "banana", "apple", "sandwich", "orange",
        "broccoli", "carrot", "hot dog", "pizza", "donut", "cake", "chair", "couch", "potted plant", "bed",
        "dining table", "toilet", "tv", "laptop", "mouse", "remote", "keyboard", "cell phone", "microwave", "oven",
        "toaster", "sink", "refrigerator", "book", "clock", "vase", "scissors", "teddy bear", "hair drier", "toothbrush"
    ];
    if id < classes.len() {
        classes[id].to_string()
    } else {
        format!("Unknown {}", id)
    }
}
