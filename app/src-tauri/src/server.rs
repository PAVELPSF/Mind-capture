use std::sync::Arc;

use crate::commands::export::get_export_payload_inner;
use crate::commands::import::{import_tabs, NewTab};
use crate::db::Database;

pub fn start(db: Arc<Database>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let server = match tiny_http::Server::http("127.0.0.1:1422") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[MindCapture] Не удалось запустить HTTP-сервер: {e}");
                return;
            }
        };

        println!("[MindCapture] HTTP-сервер слушает http://127.0.0.1:1422");

        for request in server.incoming_requests() {
            let method = request.method();
            let url = request.url().to_string();

            match (method.as_str(), url.as_str()) {
                ("POST", "/import") => handle_import(&db, request),
                ("GET", "/health") => respond_json(
                    request,
                    200,
                    r#"{"status":"ok"}"#,
                ),
                ("GET", "/export-payload") => {
                    match get_export_payload_inner(&db) {
                        Ok(payload) => {
                            let json = serde_json::to_string(&payload).unwrap();
                            respond_json(request, 200, &json);
                        }
                        Err(e) => {
                            respond_json(
                                request,
                                500,
                                &format!(r#"{{"error":"{e}"}}"#),
                            );
                        }
                    }
                }
                _ => respond_json(request, 404, r#"{"error":"not found"}"#),
            }
        }
    })
}

fn handle_import(db: &Arc<Database>, mut request: tiny_http::Request) {
    let mut body = String::new();
    if let Err(e) = request.as_reader().read_to_string(&mut body) {
        respond_json(
            request,
            400,
            &format!(r#"{{"error":"failed to read body: {e}"}}"#),
        );
        return;
    }

    let tabs: Vec<NewTab> = match serde_json::from_str(&body) {
        Ok(t) => t,
        Err(e) => {
            respond_json(
                request,
                400,
                &format!(r#"{{"error":"invalid JSON: {e}"}}"#),
            );
            return;
        }
    };

    match import_tabs(db, tabs) {
        Ok(result) => {
            let json = serde_json::to_string(&result).unwrap();
            respond_json(request, 200, &json);
        }
        Err(e) => {
            respond_json(
                request,
                500,
                &format!(r#"{{"error":"{e}"}}"#),
            );
        }
    }
}

fn respond_json(request: tiny_http::Request, status: u16, body: &str) {
    let ct = "Content-Type: application/json".parse::<tiny_http::Header>();
    let acao = "Access-Control-Allow-Origin: *".parse::<tiny_http::Header>();

    let response = tiny_http::Response::from_string(body)
        .with_status_code(status);

    let response = if let Ok(h) = ct {
        response.with_header(h)
    } else {
        response
    };

    let response = if let Ok(h) = acao {
        response.with_header(h)
    } else {
        response
    };

    let _ = request.respond(response);
}
