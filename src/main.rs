mod log;
mod ml;

use fastly::http::{Method, StatusCode};
use fastly::{Error, Request, Response};
use log::emit_log;

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    let mut resp = Response::new()
        .with_header("Access-Control-Allow-Origin", "*")
        .with_header("Access-Control-Allow-Headers", "Content-Type");
    let session = req.get_query_str().unwrap_or("session=")[8..].to_owned();
    let context = "main";
    match (req.get_method(), req.get_header_str("Content-Type")) {
        (&Method::POST, Some("image/jpeg")) => {
            emit_log(
                context,
                &session,
                "Request received. Loading model mobilenet_v2_1.4_224 (ImageNet).",
            );
            let model = include_bytes!("../models/mobilenet_v2_1.4_224_frozen.pb");
            match ml::infer(model, &req.take_body_bytes(), &session) {
                Ok((confidence, label_index)) => {
                    emit_log(
                        context,
                        &session,
                        &format!("Image classified as ImageNet label index {} (confidence {:2}).", confidence, label_index)
                    );
                    resp.set_body_text_plain(&format!("{},{}", confidence, label_index));
                }
                Err(e) => {
                    emit_log(context, &session, &format!("Inference error: {:?}", e));
                    resp.set_body_text_plain(&format!("errored: {:?}", e));
                }
            }
        }
        (&Method::OPTIONS, _) => resp.set_status(StatusCode::OK),
        _ => resp.set_status(StatusCode::IM_A_TEAPOT),
    }

    Ok(resp)
}
