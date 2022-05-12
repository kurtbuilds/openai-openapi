use std::fs::File;
use openapi_client_generator::{generate_library, GenerateLibrary, OpenAPI, read_spec};
use openapi_client_generator::openapiv3::ExternalDocumentation;

fn modify_spec(spec: &mut OpenAPI) {
    spec.paths.paths.get_mut("/engines").unwrap().as_mut().unwrap().get.as_mut().unwrap().external_docs = Some(ExternalDocumentation {
        url: "https://beta.openai.com/docs/api-reference/engines/list".to_string(),
    ..Default::default()});

}


fn main() {
    let mut spec = read_spec("openapi.yaml".as_ref()).unwrap();

    modify_spec(&mut spec);

    serde_yaml::to_writer(&mut File::create("openapi2.yaml").unwrap(), &spec).unwrap();

    generate_library(spec, GenerateLibrary {
        name: "OpenAi".to_string(),
        dest_path: "src".into(),
        lib_rs_path: Some("template/lib.rs".into()),
        model_rs_path: Some("template/model.rs".into()),
    }).unwrap();
}