#[macro_use]
extern crate neon;

use neon::prelude::*;
use safe_cli::{Safe, SafeData};

const APP_ID: &str = "net.maidsafe.safe_nodejs";

fn convert_vec_to_array(mut cx: FunctionContext, vec: Vec<u8>) -> JsResult<JsArray> {
    // Create the JS array
    let js_array = JsArray::new(&mut cx, vec.len() as u32);

    // Iterate over the rust Vec and map each value in the Vec to the JS array
    for (i, obj) in vec.iter().enumerate() {
        let js_number = cx.number(*obj);
        js_array.set(&mut cx, i as u32, js_number).unwrap();
    }

    Ok(js_array)
}

fn fetch(mut cx: FunctionContext) -> JsResult<JsArray> {
    let mut safe = Safe::new("base32z");
    let _ = safe.connect(APP_ID, None);

    let url = cx.argument::<JsString>(0)?.value();
    println!("Fetching from: {}", url);
    let response = safe.fetch(&url).unwrap_or_else(|err| { panic!(format!("Failed to fetch content: {:?}", err)) } );

    let data = match response {
        SafeData::PublishedImmutableData {
            data,
            xorname: _,
            resolved_from: _,
        } => {
            println!("Raw content of the file: {:?}", data);
            convert_vec_to_array(cx, data)
        }
        other => {
            println!("SafeData: {:?}", other);
            panic!("Unexpected SafeData");
        },
    };

    data
}

register_module!(mut cx, { cx.export_function("fetch", fetch) });

// Temporary patch to have it work for electron v6
#[no_mangle]
pub extern "C" fn __cxa_pure_virtual() {
    loop {}
}
