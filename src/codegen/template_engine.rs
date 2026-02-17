use super::visitor::SymbolTable;
use minijinja::{Environment, context};
use std::fs::{self, File};
use std::io::{Write, Error};
use std::collections::HashMap;
use std::path::Path;

pub fn create_files(symbol_table: &mut SymbolTable) -> Result<(), Error> {
    // dirty_filename is the path fit singel quotes
    for (dirty_filename, classes) in &symbol_table.records {
        let filename = dirty_filename.trim_matches('\'').to_string();
        println!("{:?}", filename);
        
        let file_path = format!("{}.txt", filename);
        let path = Path::new(&file_path);
        
        if let Some(parent_dir) = path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        
        let mut file = File::create(&file_path)?;

        writeln!(file, "Symbol Table for File: {}", filename)?;
        writeln!(file, "================================")?;

        for (class_name, class_record) in classes {
            writeln!(file, "\nClass: {}", class_name)?;
            
            for (method_name, method_record) in &class_record.methods {
                let params_list = method_record.params.join(", ");
                writeln!(file, "  - Method: {}({})", method_name, params_list)?;
            }
        }
        
        println!("Archivo '{}' generado con éxito.", filename);
    }

    Ok(())
}