extern crate iron;
extern crate typed_html;
extern crate typed_html_macros;
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric, Uniform, Standard};

use iron::prelude::*;
use iron::status;

use typed_html::elements::FlowContent;
use typed_html::types::LinkType;
use typed_html::{dom::DOMTree, html, text, OutputType};

use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::*;
use std::error::Error;

use std::process::Command;

struct Html(DOMTree<String>);

pub fn transpile(json: serde_json::value::Value) -> Response {
    let error_msg = "Error: json[code] is not a string.";
    let code = json["code"].as_str().unwrap_or(error_msg);

    if code.eq(error_msg){
        return Response::with((status::BadRequest, error_msg));
    }

    let a = "fn main() {\
  println!(\"Hello World!\");\
}";
    eval(a);

    let doc: DOMTree<String> = html!(
        <div>{ text!(code) }</div>
    );

    return Response::with((status::Ok, doc.to_string()));
}

fn eval(code: &str) -> String {
    let rand: String = thread_rng().sample_iter(&Alphanumeric).take(60).collect();
    let path_str: String = format!("data/eval_{}.rs", rand);
    let path_exec: String = format!("data/eval_{}", rand);
    let path = Path::new(&path_str);

    create_src_file(code, path);
    if !compile_file(&path_str) {
        // TODO: Error handling
    }

    let result = eval_file(&path_exec);
    remove_files(&path_str, &path_exec);
    println!("{}", result);

    return result;
}

fn create_src_file(code: &str, path: &Path) {
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    match file.write_all(code.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why.description()),
        Ok(_) => info!("successfully wrote to {}", path.display()),
    }
}

fn compile_file(path: &String) -> bool {
    let output = Command::new("/Users/julianbrendl/.cargo/bin/rustc")
        .arg(path)
        .arg("--out-dir")
        .arg("data")
        .status()
        .expect("Error");

    return output.success();
}

fn eval_file(path: &String) -> String {
    let cmd = format!("./{}", path);
    let output = Command::new(cmd)
        .output()
        .expect("Error evaluating code.");

    return String::from_utf8(output.stdout).unwrap_or("Error evaluating code.".to_string());
}

fn remove_files(src_file: &String, bin_file: &String) {
    fs::remove_file(src_file);
    fs::remove_file(bin_file);
}

