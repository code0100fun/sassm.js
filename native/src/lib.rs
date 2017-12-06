#[macro_use]
extern crate neon;
extern crate sassers;

mod string_writer;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use std::string::String;

fn compile(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let filename_string = call.arguments.require(scope, 0)?.check::<JsString>()?.value();
    let input_filename: &str = &filename_string;
    let style = "nested";
    let mut output = string_writer::StringWriter { value: String::new() };
    sassers::compile(input_filename, &mut output, style).unwrap_or_else(|e| {
        println!("Compilation failed: {}", e.message);
    });
    Ok(JsString::new(scope, &output).unwrap())
}

register_module!(m, {
    m.export("compile", compile)
});
