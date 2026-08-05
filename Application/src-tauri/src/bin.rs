use ort::session::{Session, builder::GraphOptimizationLevel};
use ort::value::Tensor;

fn main() {
    ort::init().with_name("YOLOv8").commit().unwrap();
    let session = Session::builder().unwrap()
        .with_optimization_level(GraphOptimizationLevel::Level3).unwrap()
        .with_intra_threads(4).unwrap()
        .commit_from_file("assets/yolov8n.onnx").unwrap();
}
