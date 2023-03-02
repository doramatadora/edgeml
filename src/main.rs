mod ml;

use fastly::http::{Method, StatusCode};
use fastly::{Error, Request, Response, ObjectStore};

const ML_MODEL: &str = "mobilenet_v2_1.4_224";

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // `models` Object Store.
    let models = ObjectStore::open("models")?.unwrap();

    let mut resp = Response::new()
        .with_header("Access-Control-Allow-Origin", "*")
        .with_header("Access-Control-Allow-Headers", "Content-Type");

    match (req.get_method(), req.get_header_str("Content-Type")) {
        (&Method::POST, Some("image/jpeg")) => {
            // To use a model, load it from the Object Store.
            match models.lookup_bytes(ML_MODEL) {
                Ok(Some(model)) => {
                    println!("Loaded model {} from Object Store.", ML_MODEL);
                    match ml::infer(&model, &req.take_body_bytes()) {
                        Ok((confidence, label_index)) => {
                            println!("Image classified! ImageNet label index {} (confidence {:2}).", label_index, confidence);
                            resp.set_body_text_plain(&format!("{},{}", confidence, label_index));
                        }
                        Err(e) => {
                            eprintln!("Inference error: {:?}", e);
                            resp.set_body_text_plain(&format!("errored: {:?}", e));
                        }
                    }
                },
                _ => {
                    resp.set_status(StatusCode::INTERNAL_SERVER_ERROR);
                    resp.set_body_text_plain(&format!("Failed to load model {} from Object Store.", ML_MODEL));
                }
            };
            
        }
        (&Method::OPTIONS, _) => resp.set_status(StatusCode::OK),
        _ => resp.set_status(StatusCode::IM_A_TEAPOT),
    }

    Ok(resp)
}
