use super::visitor::{SymbolTable, ClassRecord};
use minijinja::{Environment, context};
use std::fs::{self, File};
use std::io::{Write, Error};
use std::collections::HashMap;
use std::path::Path;

pub fn create_files(symbol_table: &mut SymbolTable) -> Result<(), Error> {
    // dirty_filename is the path with single quotes
    for (dirty_filename, classes) in &symbol_table.records {
        let mut filename = dirty_filename.trim_matches('\'').to_string();
        filename = filename.strip_prefix("./").unwrap_or(&filename).to_string();
        
        let client_path = format!("result/client/{}.js", filename);
        let server_path = format!("result/server/{}.js", filename);
        create_client_files(&client_path, &classes);
        create_server_files(&server_path, &classes);
        
        println!("Archivo '{}' generado con éxito.", filename);
    }

    Ok(())
}

fn create_client_files(file_path: &String, classes: &HashMap<String, ClassRecord>)-> Result<(), Error>{
    let path = Path::new(&file_path);
    
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let client_conn_loc = file_path.matches('/').count() - 2;
    let mut loc_dots= String::new();
    for _ in 0..=client_conn_loc{
        loc_dots.push('.');
    };

    let mut file = File::create(&file_path)?;

    let mut env = Environment::new();
    env.add_template("class", "import {ClientConnector} from '{{ loc_dots }}/ClientConnector.js'

export class {{ className }} extends ClientConnector{
{{ methods }}
}").unwrap();

    let class_template = env.get_template("class").unwrap();
    let class_render = class_template.render(context! {
        loc_dots => loc_dots,
        className => "test",
        methods => "test2"
    }).unwrap();
    writeln!(file, "{}" , class_render);

    for (class_name, class_record) in classes {
        writeln!(file, "\nClass: {}", class_name)?;
        
        for (method_name, method_record) in &class_record.methods {
            let params_list = method_record.params.join(", ");
            writeln!(file, "  - Method: {}({})", method_name, params_list)?;
        }
    }
    Ok(())
}

fn create_server_files(file_path: &String, classes: &HashMap<String, ClassRecord>)-> Result<(), Error>{
    let path = Path::new(&file_path);
    
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    
    let mut file = File::create(&file_path)?;

    writeln!(file, "Symbol Table for File: {}", file_path)?;
    writeln!(file, "================================")?;

    for (class_name, class_record) in classes {
        writeln!(file, "\nClass: {}", class_name)?;
        
        for (method_name, method_record) in &class_record.methods {
            let params_list = method_record.params.join(", ");
            writeln!(file, "  - Method: {}({})", method_name, params_list)?;
        }
    }
    Ok(())
}