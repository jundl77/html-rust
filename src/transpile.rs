extern crate iron;
extern crate typed_html;
extern crate typed_html_macros;

use iron::prelude::*;
use iron::status;

use typed_html::elements::FlowContent;
use typed_html::types::LinkType;
use typed_html::{dom::DOMTree, html, text, OutputType};

struct Html(DOMTree<String>);

pub fn transpile(json: serde_json::value::Value) -> Response {
    let error_msg = "Error: json[code] is not a string.";
    let code = json["code"].as_str().unwrap_or(error_msg);

    if code.eq(error_msg){
        return Response::with((status::BadRequest, error_msg));
    }

    let doc: DOMTree<String> = html!(
        <div>{ text!(code) }</div>
    );

    return Response::with((status::Ok, doc.to_string()));
}

