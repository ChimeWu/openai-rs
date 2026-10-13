use openai_api_types::responses as protocol_responses;
use openai_rs::types;

fn sdk_path_to_protocol_path(value: types::CreateResponse) -> protocol_responses::CreateResponse {
    value
}

#[test]
fn sdk_types_are_reexports_of_protocol_crate_types() {
    let request = types::CreateResponse::builder().try_into().unwrap();
    let _: protocol_responses::CreateResponse = sdk_path_to_protocol_path(request);
}
