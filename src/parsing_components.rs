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
    Ok(result)
}

/**
 * return -1 if parsing failed
 */
pub fn parse_time_in_ms (time_str : &str) -> i64{
    let time_number = time_str.strip_suffix(" ms");
    if time_number.is_some(){
    time_number.unwrap().parse().expect("Failed to parse time")
    } else { -1 }
}


mod parse_dates_from_filenames{
	use chrono::DateTime;
	use std::fs::PathBuf;

	
	pub fn obtain_date_of_file (filepath : &PathBuf ) -> Result<DateTime<Local>, Error>{
		//first try meta info
		match date_from_meta_info(filepath){
			//if that failes -> parse filename
			Err(_) => parse_date_from_file_name(&filepath),
			
			Ok()
		}
	}
	
	
	fn date_from_meta_info(filepath : &PathBuf) -> Result<DateTime<Local>, Error>{
		std::fs::metadata(filepath)?.mmodified()?
	}
	
	fn parse_date_from_file_name(filepath : &PathBuf) -> Result<DateTime<Local>, Error>{
		// 	blablablablabla-YYYY-MM-DD-HH-mm-ss
		//	profiler-report-2021-12-09-11-35-14

		let file_name_pattern = "%Y-%m-%d-%H-%M-%S";
		
		DateTime::parse_from_str(filepath.filename(), file_name_pattern) //todo: slicing by hand?
	}
}