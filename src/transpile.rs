extern crate iron;
extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric, Uniform, Standard};

use iron::prelude::*;
use iron::status;

use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::*;
use std::error::Error;

use std::process::Command;

const PRELUDE: &str = "
extern crate typed_html;

use typed_html::elements::FlowContent;
use typed_html::types::LinkType;
use typed_html::{dom::DOMTree, html, text, OutputType};

fn main() {
    println!(\"{}\", render().to_string());
}
";

const TEST: &str = "
fn render() -> DOMTree<String> {
    let code = \"HELLO WORLD!\";

    return html!(
        <div>{ text!(code) }</div>
    );
}
";

pub fn transpile(json: serde_json::value::Value) -> Option<String> {
    let error_msg = "Error: json[code] is not a string.";
    let code = json["code"].as_str().unwrap_or(error_msg);

    if code.eq(error_msg){
        return None;
    }

    let complete_code: String = format!("{}\n{}", PRELUDE, code);
    return eval(complete_code.as_str());
}

fn eval(code: &str) -> Option<String> {
    let rand: String = thread_rng().sample_iter(&Alphanumeric).take(60).collect();

    create_src_file(code, &rand);

    if !compile_file(&rand) {
        remove_files(&rand);
        return None;
    }

    let result = eval_file(&rand);
    remove_files(&rand);

    return result;
}

fn create_src_file(code: &str, rand: &String) {
    let path_str: String = format!("data/eval_{}.rs", rand);
    let path = Path::new(&path_str);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    match file.write_all(code.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why.description()),
        Ok(_) => info!("successfully wrote to {}", path.display()),
    }
}

fn compile_file(rand: &String) -> bool {
    let path: String = format!("data/eval_{}.rs", rand);
    let crate_name: String = format!("{}_crate", rand);

    let output = Command::new("/Users/julianbrendl/.cargo/bin/rustc")
        .arg(path)
        .args(&["--crate-name", crate_name.as_str(), "--crate-type", "bin", "--out-dir", "data"])
        .args(&["--emit=dep-info,link", "-C", "debuginfo=2", "-C", "incremental=data/typed-html/target/release/incremental"])
        .args(&["-L", "dependency=data/typed-html/target/release/deps", "--extern", "typed_html=data/typed-html/target/release/libtyped_html.rlib"])
        .status()
        .expect("Error");

    return output.success();
}

fn eval_file(rand: &String) -> Option<String> {
    let path_exec: String = format!("data/{}_crate", rand);
    let error = "Error";

    let cmd = format!("./{}", path_exec);
    let output = Command::new(cmd)
        .output()
        .expect(error);

    let result = String::from_utf8(output.stdout).unwrap();

    if result != error {
        return Some(result);
    }
    return None;
}

fn remove_files(rand: &String) {
    fs::remove_file(format!("data/eval_{}.rs", rand));
    fs::remove_file(format!("data/{}_crate", rand));
    fs::remove_file(format!("data/{}_crate.d", rand));
    fs::remove_dir_all(format!("data/{}_crate.dSYM", rand));
}

