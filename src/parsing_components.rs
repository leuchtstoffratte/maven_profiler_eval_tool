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


pub mod parse_dates_from_filenames{
	use chrono::{NaiveDateTime,  ParseError};
	use std::path::PathBuf;
	use std::io::Error;


	
	pub fn obtain_date_of_file (filepath : &PathBuf ) -> Result<NaiveDateTime, ParseError>{
			parse_date_from_file_name(&filepath)
	}
	
	#[allow(dead_code)]
	fn date_from_meta_info(filepath : &PathBuf) -> Result<chrono::DateTime<chrono::Local>, Error>{
		match std::fs::metadata(filepath)?.modified(){
			Ok(t) => Ok(chrono::DateTime::from(t)),
			Err(e) => Err(e)
		}
	}
	
	fn parse_date_from_file_name(filepath : &PathBuf) -> Result<NaiveDateTime, ParseError>{
		// 	_______________-YYYY-MM-DD-HH-mm-ss
		//	profiler-report-2021-12-09-11-35-14.json

		let file_name_pattern = "%Y-%m-%d-%H-%M-%S";

		let date_time_of_file = match filepath.file_name()
							.and_then(std::ffi::OsStr::to_str)
							.and_then(|s| s.get(16..35))
							{
								None => "",
								Some(s) => s
							};

		NaiveDateTime::parse_from_str(date_time_of_file, file_name_pattern)

	}

}