use std::path::PathBuf;


fn main(){

    let test_dir = PathBuf::from("E:\\Rattenmann\\Coding\\Rust\\maven_profiler_summation\\real_data\\maven-profiler-reports");
    let real_dir = file_walking_and_extracting::read_directory_from_command_line().expect("A directory needs to be specified.");

    let files : Vec<PathBuf> = file_walking_and_extracting::get_list_of_json_files_in_directory(&real_dir);

    let mut results : Vec<parsing_components::MavenProfilerReport> = Vec::new();

    let mut statistic = output::create_build_summary();


    for f in files{

        let display_path = match f.to_str(){
            Some(name) => name,
            None => "failed to parse path."
        };
        println!("Now parsing:             {} ", display_path);

        let json_str = file_walking_and_extracting::extract_json_string_from_file_by_name(&f).expect("Failed to parse file.");

        if let Ok(report) = parsing_components::parse_maven_profiler_report(&json_str) {

            let build_time = parsing_components::parse_time_in_ms(&report.time);
            if build_time > 0 {
                statistic.add(report.projects.len() as i32, build_time, 0);
            }
            results.push(report);

        }


    }

    output::print_summary(&statistic);
    

}

#[allow(dead_code)] //for now
fn sum_single_project_build_time (projekt_json : &parsing_components::ProjectBuildTimeList) -> i64{
    let mut sum_of_build_times : i64  = 0;

    for m in projekt_json.mojos.iter() {
        sum_of_build_times += parsing_components::parse_time_in_ms(&m.time);

    }

    sum_of_build_times
}


mod parsing_components {
    use serde::{Deserialize, Serialize};
    use serde_json::{Result};
  
    #[derive(Serialize, Deserialize)]
    pub struct MojoBuildTime{
        pub mojo: String,
        pub time: String
    }


    #[derive(Serialize, Deserialize)]
    pub struct ProjectBuildTimeList{
        project : String,
        time : String,
        pub mojos : Vec<MojoBuildTime>
    }


    #[derive(Serialize, Deserialize)]
    pub struct MavenProfilerReport{
      pub name : String,
      profile_name : String,
      pub time : String,
      goals: String,
      date : String,
      parameters: String,
      pub projects: Vec<ProjectBuildTimeList>
    //   ,
    //   pub downloads : Vec<>
    }



    pub fn parse_maven_profiler_report( report:  &str ) -> Result<MavenProfilerReport>{

        let result : MavenProfilerReport = serde_json::from_str(report)?;
      
        println!(" paresed {}", result.name);
      
        Ok(result)
    }
    
  /**
   * return -1 if parsing failed
   */
  pub fn parse_time_in_ms (time_str : &str) -> i64{
    
      let time_number = time_str.strip_suffix(" ms");
      if time_number.is_some(){
        time_number.unwrap().parse().expect("")
      } else { -1 }
    }
  
  
}
  

mod file_walking_and_extracting{
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

}

mod output{

    pub struct Build_summary{
        number_of_builds : i32,
        total_time_spend_on_build : i64, //milliseconds 
        total_time_spend_on_downloads : i64 //milliseconds
    }

    impl Build_summary{

        pub fn add(&mut self, n_builds :i32, t_build :i64, t_download :i64){
            self.number_of_builds += n_builds;
            self.total_time_spend_on_build += t_build;
            self.total_time_spend_on_downloads += t_download;
        }
    }

    pub fn create_build_summary() -> Build_summary{
        Build_summary{
            number_of_builds : 0,
            total_time_spend_on_build : 0,
            total_time_spend_on_downloads :0
        }
    }


    pub fn print_summary(summary : &Build_summary){
        println!("--------------------------------------------------------------------");
        println!("{} projects were build.", summary.number_of_builds);
        println!("Total build time was {} secs", summary.total_time_spend_on_build/1000);
        println!("Total time spent on downloads was {} secs", summary.total_time_spend_on_downloads/1000);


    }


    pub fn export_to_excel(summary : Build_summary){
        unimplemented!()
    }
}





//TODO: i64-random number as identifier
//TODO: create file with identifier
//TODO: if id-file not found, create new Id and write file