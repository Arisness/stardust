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
        create_client_files(&client_path, &classes, &filename);
        create_server_files(&server_path, &classes);
        
        println!("Archivo '{}' generado con éxito.", filename);
    }

    Ok(())
}

fn create_client_files(file_path: &String, classes: &HashMap<String, ClassRecord>, filename: &str)-> Result<(), Error>{
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

    let server_filename = format!("./BussinesObjects/{}.js", filename);

    let mut env = Environment::new();
    env.add_template("class", "import ClientConnector from '{{ loc_dots }}/ClientConnector.js'

export class {{ className }} extends ClientConnector{
{{ methods }}
}").unwrap();

    env.add_template("method", "    async {{ methodName }}({{ params }}){
        let jsonToSend = this.createJSON(\"{{ path }}\", \"{{className}}\", \"{{methodName}}\", {{params}} );
        let jsonSerialized = this.serialize(jsonToSend);
        await this.send(jsonSerialized);
    }
    ").unwrap();

    let class_template = env.get_template("class").unwrap();

    let method_template = env.get_template("method").unwrap();


    for (class_name, class_record) in classes {
        let mut methods = String::new();
        for (method_name, method_record) in &class_record.methods {
            let params_list = method_record.params.join(", ");
            let method_render = method_template.render(context! {
                methodName => method_name,
                params => params_list,
                className =>class_name,
                path => server_filename
            }).unwrap();
            methods.push('\n');
            methods.push_str(&method_render);
        }
        let class_render = class_template.render(context! {
            loc_dots => loc_dots,
            className => class_name,
            methods => methods
        }).unwrap();
        writeln!(file, "{}" , class_render);
    }
    Ok(())
}

fn create_server_files(file_path: &String, classes: &HashMap<String, ClassRecord>)-> Result<(), Error>{
    let path = Path::new(&file_path);
    
    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let mut file = File::create(&file_path)?;


    let mut env = Environment::new();
    env.add_template("class", "
export class {{ className }}{
{{ methods }}
}").unwrap();

    env.add_template("method", "    {{ methodName }}({{ params }}){

    }
    ").unwrap();

    let class_template = env.get_template("class").unwrap();

    let method_template = env.get_template("method").unwrap();


    for (class_name, class_record) in classes {
        let mut methods = String::new();
        for (method_name, method_record) in &class_record.methods {
            let params_list = method_record.params.join(", ");
            let method_render = method_template.render(context! {
                methodName => method_name,
                params => params_list
            }).unwrap();
            methods.push('\n');
            methods.push_str(&method_render);
        }
        let class_render = class_template.render(context! {
            className => class_name,
            methods => methods
        }).unwrap();
        writeln!(file, "{}" , class_render);
    }
    Ok(())
}