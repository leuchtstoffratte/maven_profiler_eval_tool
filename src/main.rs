use std::fs;


fn main(){
    // let reading_went_well = test_read_mojo_list();

    let file_name = "E:\\Rattenmann\\Coding\\Rust\\maven_profiler_summation\\samples\\maven_profiler_sample_jsons.txt";

    let report_content = fs::read_to_string(file_name).expect("reading file failed.");
    
    let reading_went_well = crate::parsing_components::parse_maven_profiler_report(&report_content).expect("extracting json data failed");

    let project_name = reading_went_well.name;
    let n_project = reading_went_well.projects.len();
    let build_duration = sum_single_project_build_time(&reading_went_well.projects[0]);


    println!("Project {} contained {} subprojects and they took {} ms to build.", project_name,n_project, build_duration);


    
}





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
      time : String,
      goals: String,
      date : String,
      parameters: String,
      pub projects: Vec<ProjectBuildTimeList>
    }



    pub fn parse_maven_profiler_report( report:  &str ) -> Result<MavenProfilerReport>{

        let result : MavenProfilerReport = serde_json::from_str(report)?;
      
        println!(" paresed {}", result.name);
      
        Ok(result)
    }
    
    
    

  /**
   * return -10 if parsing failed
   */
  pub fn parse_time_in_ms (time_str : &str) -> i64{
    
      let time_number = time_str.strip_suffix(" ms");
      if time_number.is_some(){
        time_number.unwrap().parse().expect("")
      } else { -1 }
    }
  
  
}
  
  