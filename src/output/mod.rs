use chrono::prelude::*;

pub struct BuildSummary{
    number_of_builds : i32,          //this is the number of maven profiler-reports
    number_of_build_projects: i32,   //this is the number of (non unique) projects that were build
    total_time_spend_on_build : i64, //milliseconds 
    total_time_spend_on_downloads : i64 //milliseconds
	oldest_included_build : DateTime<Local> 	 //date time of the oldest build json that was included in statistic
}

impl BuildSummary{

    pub fn add_results_from_one_maven_run(&mut self, n_builds :i32, t_build :i64, t_download :i64, date_of_build : DateTime<Local>){
        self.number_of_builds += 1; //asumption, but seems to be a very good one
        self.number_of_build_projects += n_builds;
        self.total_time_spend_on_build += t_build;
        self.total_time_spend_on_downloads += t_download;
		
		if ( date_of_build < self.oldest_included_build  ) {
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
		oldest_included_build : Local::now()
    }
}


pub fn print_summary(summary : &BuildSummary){
    let build_secs = summary.total_time_spend_on_build/1000;
    let build_hours = (summary.total_time_spend_on_build as f64)/3600000.;
    println!("--------------------------------------------------------------------");
    println!("maven was called on {} occasions since {}.", summary.number_of_builds, summary.oldest_included_build.date());
    println!("{} projects were build.", summary.number_of_build_projects);
    println!("Total build time was {number:>width$} secs", number=build_secs, width=8);
    println!("                   = {number:>width$.2} hours", number=build_hours, width=8);
    println!("Total time spent on downloads was {} secs", summary.total_time_spend_on_downloads/1000);

}



#[allow(dead_code)] //for now
pub fn export_to_excel(summary : BuildSummary){
    unimplemented!()
}
