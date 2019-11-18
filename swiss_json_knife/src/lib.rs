use actix_web::{web, HttpResponse};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn index(input: web::Path<(String, String)>) -> Result<HttpResponse> {
    let resolved_filename = get_filename(&input.0);
    let columns = get_columns(&input.1);
    let data = get_string_from_file(resolved_filename);
    let json_input: Value = serde_json::from_str({ &data }).expect("unable to parse");
    let json_object = json_input.as_object().unwrap();
    let mut output_container = Vec::new();
    for (_k, v) in json_object.iter() {
        let mut output: HashMap<String, &serde_json::value::Value> = HashMap::new();
        if v.is_array() {
            for value in v.as_array().unwrap() {
                if value.is_object() {
                    let sub_object = value.as_object().unwrap();
                    for (sub_key, sub_val) in sub_object.iter() {
                        if columns.contains(sub_key) || columns[0] == "-" {
                            output.insert(sub_key.to_string(), sub_val);
                        }
                    }
                }
                output_container.push(output.clone());
            }
        } else if v.is_object() {
            let sub_object = v.as_object().unwrap();
            for (sub_key, sub_val) in sub_object.iter() {
                output.insert(sub_key.to_string(), sub_val);
            }
        }
    }
    Ok(HttpResponse::Ok().json(output_container))
}

pub fn get_string_from_file(filename: String) -> String {
    let mut json_file = File::open(filename).expect("not found");
    let mut data = String::new();
    json_file.read_to_string(&mut data).expect("wtf");
    data
}

pub fn get_columns(input: &String) -> Vec<String> {
    let split = input.split(",");
    let mut columns = Vec::new();
    for s in split {
        columns.push(s.to_string());
    }
    columns
}

pub fn get_filename(input: &String) -> String {
    let filename_split = input.split("-");
    let mut resolved_filename = String::new();
    for s in filename_split {
        resolved_filename.push_str("/");
        resolved_filename.push_str(s);
    }
    resolved_filename
}
