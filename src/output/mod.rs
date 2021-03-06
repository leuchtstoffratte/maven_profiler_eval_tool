use chrono::{NaiveDateTime, NaiveDate};

fn version_number () -> String{
    String::from("0.11")
}

pub struct BuildSummary{
    number_of_builds : i32,                     //this is the number of maven profiler-reports
    number_of_build_projects: i32,              //this is the number of (non unique) projects that were build
    total_time_spend_on_build : i64,            //milliseconds 
    total_time_spend_on_downloads : i64,        //milliseconds
	oldest_included_build : NaiveDateTime    	//date time of the oldest build json that was included in statistic
}

impl BuildSummary{

    pub fn add_results_from_one_maven_run(&mut self, n_builds :i32, t_build :i64, t_download :i64, date_of_build : NaiveDateTime){
        self.number_of_builds += 1; //asumption, but seems to be a very good one
        self.number_of_build_projects += n_builds;
        self.total_time_spend_on_build += t_build;
        self.total_time_spend_on_downloads += t_download;
		
		if  date_of_build < self.oldest_included_build   {
			self.oldest_included_build = date_of_build;
		}
    }
}

pub fn create_build_summary() -> BuildSummary{
    BuildSummary{
        number_of_builds : 0,
        number_of_build_projects : 0,
        total_time_spend_on_build : 0,
        total_time_spend_on_downloads : 0,
		oldest_included_build : NaiveDate::from_ymd(2300, 12,31).and_hms(23,59,59)
    }
}


pub fn print_summary(summary : &BuildSummary){
    let build_secs = summary.total_time_spend_on_build/1000;
    let build_hours = (summary.total_time_spend_on_build as f64)/3600000.;
    println!("maven-profiler-eval-tool: Version {}", version_number());
    println!("--------------------------------------------------------------------");
    println!("maven was called on {} occasions since {}.", summary.number_of_builds, summary.oldest_included_build.date());
    println!("{} projects were build.", summary.number_of_build_projects);
    println!("Total build time was {number:>width$} secs", number=build_secs, width=8);
    println!("                   = {number:>width$.2} hours", number=build_hours, width=8);

}
