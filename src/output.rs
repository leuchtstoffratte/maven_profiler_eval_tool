pub struct BuildSummary{
    number_of_builds : i32,
    total_time_spend_on_build : i64, //milliseconds 
    total_time_spend_on_downloads : i64 //milliseconds
}

impl BuildSummary{

    pub fn add(&mut self, n_builds :i32, t_build :i64, t_download :i64){
        self.number_of_builds += n_builds;
        self.total_time_spend_on_build += t_build;
        self.total_time_spend_on_downloads += t_download;
    }
}

pub fn create_build_summary() -> BuildSummary{
    BuildSummary{
        number_of_builds : 0,
        total_time_spend_on_build : 0,
        total_time_spend_on_downloads :0
    }
}


pub fn print_summary(summary : &BuildSummary){
    println!("--------------------------------------------------------------------");
    println!("{} projects were build.", summary.number_of_builds);
    println!("Total build time was {} secs", summary.total_time_spend_on_build/1000);
    println!("Total time spent on downloads was {} secs", summary.total_time_spend_on_downloads/1000);


}

#[allow(dead_code)] //for now
pub fn export_to_excel(summary : BuildSummary){
    unimplemented!()
}
