fn main() {
    let processes_output = std::process::Command::new("sh")
        .arg("-c")
        .arg("ps aux | awk '{ print $4 }'")
        .output()
        .expect("Failed to get mem usage data");

    let mem_usage_list = String::from_utf8(processes_output.stdout).expect("Failed to parse command output");

    let mut total = 0_f64;

    for mem_percent in mem_usage_list.split("\n") {
        match mem_percent.parse::<f64>() {
            Ok(percent) => total += percent,
            Err(_) => continue
        }
    }

    println!("{}", total)
}
