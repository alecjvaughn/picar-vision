import SwiftRs
import Tauri
import UIKit
import WebKit
import Vision
import CoreML

class InferenceArgs: Decodable {
  let imageBase64: String
}

class CoreMLPlugin: Plugin {
    var visionModel: VNCoreMLModel?
    
    override init() {
        super.init()
        self.setupModel()
    }
    
    private func setupModel(invoke: Invoke? = nil) {
        var url = Bundle.main.url(forResource: "yolov8n", withExtension: "mlmodelc", subdirectory: "assets")
        if url == nil {
            url = Bundle.main.url(forResource: "yolov8n", withExtension: "mlmodelc")
        }
        if url == nil {
            url = Bundle.main.url(forResource: "assets/yolov8n", withExtension: "mlmodelc")
        }
        if url == nil {
            url = Bundle.main.url(forResource: "assets/yolov8n.mlmodelc", withExtension: nil)
        }
        
        guard let modelURL = url else {
            let fm = FileManager.default
            let bundleRoot = Bundle.main.bundlePath
            let contents = try? fm.contentsOfDirectory(atPath: bundleRoot)
            let assetsContents = try? fm.contentsOfDirectory(atPath: bundleRoot + "/assets")
            let debugStr = "Model not found. Root: \(contents?.prefix(5).description ?? "none"). Assets: \(assetsContents?.description ?? "none")"
            print(debugStr)
            invoke?.reject(debugStr)
            return
        }
        do {
            let mlModel = try MLModel(contentsOf: modelURL)
            self.visionModel = try VNCoreMLModel(for: mlModel)
        } catch {
            print("Error loading CoreML model: \(error)")
        }
    }
    
    @objc public func runInference(_ invoke: Invoke) throws {
        if self.visionModel == nil {
            self.setupModel(invoke: invoke)
        }
        guard let visionModel = self.visionModel else {
            // Error is already sent by setupModel(invoke:)
            return
        }
        
        let args = try invoke.parseArgs(InferenceArgs.self)
        guard let imageData = Data(base64Encoded: args.imageBase64),
              let image = UIImage(data: imageData),
              let cgImage = image.cgImage else {
            invoke.reject("Invalid image data")
            return
        }
        
        let request = VNCoreMLRequest(model: visionModel) { (request, error) in
            if let error = error {
                invoke.reject(error.localizedDescription)
                return
            }
            
            var resultsArray: [[String: Any]] = []
            
            if let results = request.results as? [VNRecognizedObjectObservation] {
                for observation in results {
                    guard let topLabelObservation = observation.labels.first else { continue }
                    
                    let confidence = topLabelObservation.confidence
                    let label = topLabelObservation.identifier
                    
                    let boundingBox = observation.boundingBox
                    // Convert bottom-left origin to top-left
                    let x = boundingBox.origin.x
                    let y = 1.0 - boundingBox.origin.y - boundingBox.size.height
                    let width = boundingBox.size.width
                    let height = boundingBox.size.height
                    
                    resultsArray.append([
                        "label": label,
                        "confidence": confidence,
                        "x": x,
                        "y": y,
                        "width": width,
                        "height": height
                    ])
                }
                invoke.resolve(["boxes": resultsArray])
            } else {
                // Not recognized object format, maybe MLMultiArray
                invoke.resolve(["boxes": resultsArray])
            }
        }
        
        let handler = VNImageRequestHandler(cgImage: cgImage, options: [:])
        DispatchQueue.global(qos: .userInitiated).async {
            do {
                try handler.perform([request])
            } catch {
                invoke.reject("Failed to perform Vision request: \(error)")
            }
        }
    }
}

@_cdecl("init_plugin_coreml")
func initPlugin() -> Plugin {
  return CoreMLPlugin()
}
