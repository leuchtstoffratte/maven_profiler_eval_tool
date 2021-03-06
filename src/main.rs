use std::path::PathBuf;

mod parsing_components;
mod file_walking_and_extracting;
mod output;

fn main(){

    let real_dir = file_walking_and_extracting::read_directory_from_command_line();
    
    if real_dir.is_none(){
        println!("Please specify a path as first parameter.");
        return ;
    }
    

    let files : Vec<PathBuf> = file_walking_and_extracting::get_list_of_json_files_in_directory(&real_dir.unwrap());

    let mut results : Vec<parsing_components::MavenProfilerReport> = Vec::new();

    let mut statistic = output::create_build_summary();


    for f in files{


        let json_str = file_walking_and_extracting::extract_json_string_from_file_by_name(&f).expect("Failed to parse file.");

		let build_date_time = parsing_components::parse_dates_from_filenames::obtain_date_of_file(&f).expect("Could not parse Date from filename.");

        if let Ok(report) = parsing_components::parse_maven_profiler_report(&json_str) {

            let build_time = parsing_components::parse_time_in_ms(&report.time);
            if build_time > 0 {
                statistic.add_results_from_one_maven_run(report.projects.len() as i32, build_time, 0, build_date_time);
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


//TODO: i64-random number as identifier
//TODO: create file with identifier
//TODO: if id-file not found, create new Id and write file