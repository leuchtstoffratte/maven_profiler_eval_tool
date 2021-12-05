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


