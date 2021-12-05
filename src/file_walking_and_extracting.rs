
use std::fs::{DirEntry, read_to_string, read_dir};
use std::path::PathBuf;
use std::env;

pub fn get_list_of_json_files_in_directory ( directory_name : &PathBuf ) -> Vec<PathBuf>{
    let mut json_file_names : Vec<PathBuf> = Vec::new();

    let directroy = read_dir(directory_name);

    match directroy{
        Ok(file_list) =>{
            for file_item in file_list{
                add_json_file_names_to_list(&mut json_file_names, &file_item);
            }
        }
        Err(e) => println!("Could not access files")
    }

    json_file_names

}


pub fn extract_json_string_from_file_by_name(file_name : &PathBuf ) -> Result<String, std::io::Error> {
    read_to_string(file_name)
}



pub fn read_directory_from_command_line() -> Option<PathBuf>{

    let args : Vec<String> = env::args().collect();

    if args.len() > 1 {
        Some(PathBuf::from(&args[1]))
    }else{
        None
    }
}


fn add_json_file_names_to_list( file_names : &mut Vec<PathBuf>, 
                                file: &Result<DirEntry, std::io::Error> ){
    if let Ok(file_name) =  file {
        if is_this_a_json_file(file_name){
            file_names.push(file_name.path());
        }
    }   
}

fn is_this_a_json_file( dir_entry : &DirEntry ) -> bool{

    match dir_entry
            .file_name()
            .into_string() {
        Ok(file_name) => file_name.contains("json"),
        Err(e) => false
    }

}

