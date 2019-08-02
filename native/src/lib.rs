#[macro_use]
extern crate neon;
// extern crate num_cpus;
extern crate safe_cli;

use safe_cli::Safe;
use safe_cli::SafeData;

use neon::prelude::*;
const APP_ID: &str = "net.maidsafe.neon";

fn convert_vec_to_array(mut cx: FunctionContext, vec: Vec<u8>) -> JsResult<JsArray> {
    // let vec: Vec<String> = Vec::with_capacity(100);

    // Create the JS array
    let js_array = JsArray::new(&mut cx, vec.len() as u32);

    // Iterate over the rust Vec and map each value in the Vec to the JS array
    for (i, obj) in vec.iter().enumerate() {
        let js_number = cx.number(*obj);
        js_array.set(&mut cx, i as u32, js_number).unwrap();
    }

    Ok(js_array)
}

fn thread_count(mut cx: FunctionContext) -> JsResult<JsArray> {
    // Ok(cx.number(num_cpus::get() as f64))

	let mut safe = Safe::new("base32z".to_string());
	safe.connect(APP_ID, None);

	let response = safe.fetch("safe://kakkakakakakaka");

	let typ = match response {
		Ok(string ) => string,
		Err(err) => panic!(format!("err in fetch: {:?}", err))
	};

	let data = match typ {
		SafeData::PublishedImmutableData {
            data,
            xorname,
            resolved_from,
        } => {

                println!("Raw content of the file:");
				let js_array = convert_vec_to_array(cx, data);

				js_array
            },
		_ => panic!("upsssssssssssssssssssssss")
	};

	data
	// Ok(data)
}

register_module!(mut cx, {
    cx.export_function("threadCount", thread_count)
});
