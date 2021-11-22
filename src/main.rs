use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};


#[derive(Serialize, Deserialize)]
struct MojoBuildTime{
    mojo: String,
    time: String
}


#[derive(Serialize, Deserialize)]
struct ProjectBuildTimeList{
    project : String,
    time : String,
    mojos : Vec<MojoBuildTime>
}

fn main(){
    let reading_went_well = test_read_mojo_list();

    if reading_went_well.is_ok(){
        println!("yeah");
    }else{
        println!("möö");
    }
}




fn test_read_mojo() -> Result<()> {

    let test_mojo = r#"       {
        "mojo": "org.apache.maven.plugins:maven-compiler-plugin:3.6.0:testCompile {execution: default-testCompile}",
        "time": "2 ms"
      }"#;


    let m :Value = serde_json::from_str(test_mojo)?;


    println!("{} took {} to build", m["mojo"], m["time"]);

    Ok(())
}



fn test_read_mojo_list ()-> Result<()>{

    let mojo_list_str = r#"    {
        "project": "maven-profiler",
        "time": "43378 ms",
        "mojos": [
          {
            "mojo": "org.apache.maven.plugins:maven-invoker-plugin:2.0.0:run {execution: integration-test}",
            "time": "30706 ms"
          },
          {
            "mojo": "org.apache.maven.plugins:maven-surefire-plugin:2.19.1:test {execution: default-test}",
            "time": "7300 ms"
          },
          {
            "mojo": "org.apache.maven.plugins:maven-shade-plugin:2.4.3:shade {execution: default}",
            "time": "1378 ms"
          },
          {
            "mojo": "org.apache.maven.plugins:maven-compiler-plugin:3.6.0:compile {execution: default-compile}",
            "time": "1112 ms"
          },
          {
            "mojo": "org.codehaus.gmavenplus:gmavenplus-plugin:1.5:testCompile {execution: default}",
            "time": "1102 ms"
          },
          {
            "mojo": "org.apache.maven.plugins:maven-invoker-plugin:2.0.0:install {execution: integration-test}",
            "time": "293 ms"
          },
          {
            "mojo": "org.apache.maven.plugins:maven-enforcer-plugin:1.4.1:enforce {execution: enforce-maven}",
            "time": "225 ms"
          }
        ]
      }"#;

      let mojo_list : ProjectBuildTimeList = serde_json::from_str(mojo_list_str)?;

      let mut sum_build_times : i64 = 0;


      for mojo in mojo_list.mojos.iter(){
        sum_build_times += parse_time_in_ms(&mojo.time);
        println!("Mojo {} took {} to build", mojo.mojo, mojo.time);
      }


      println!(" project {} took {} to build. It contains {} separate mojos", mojo_list.project, mojo_list.time, mojo_list.mojos.len());

      println!(" the entire build time was {} ms ", sum_build_times);

      Ok(())
}




/**
 * return -10 if parsing failed
 */
fn parse_time_in_ms (time_str : &str) -> i64{
  
  let time_number = time_str.strip_suffix(" ms");
  if time_number.is_some(){
    time_number.unwrap().parse().expect("")
  } else { -1 }
}


fn sum_single_project_build_time (projekt_json : ProjectBuildTimeList) -> i64{
  0
}