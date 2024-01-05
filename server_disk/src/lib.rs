pub mod my_server_disk {

    #[derive(Debug, PartialEq)]
    pub struct DiskInfo {
        pub total: i32,
        pub used: i32,
        pub free: i32,
        pub percent: f32,
        pub mount_on: String,
        pub filesystem: String,
        pub hostname: String,
    }

    impl DiskInfo {
        // return a boolean value for over 90% or not
        pub fn get_server_data() -> Self {
            // use crates io to get my local ip address
            println!("I am running in the server disk crate");

            let output = std::process::Command::new("df")
                .arg("-h")
                .output()
                .expect("failed to execute process");

            let output = String::from_utf8_lossy(&output.stdout);
            println!("{}", output);

            let mut over_threadhold_server = DiskInfo {
                total: 0,
                used: 0,
                free: 0,
                percent: 0.0,
                mount_on: "".to_string(),
                filesystem: "".to_string(),
                hostname: "".to_string(),
            };

            // if Mounted on is / then get the Use%
            for line in output.lines() {
                let line = line.trim();
                let line = line.split_whitespace().collect::<Vec<&str>>();
                if line[5] == "/"{
                    over_threadhold_server.total = line[3]
                        .replace("Gi", "")
                        .replace("Ki", "")
                        .replace("Mi", "")
                        .replace("Bi", "")
                        .replace("G", "")
                        .replace("M", "")
                        .parse::<i32>().unwrap();
                    over_threadhold_server.used = line[4].replace("%", "").parse::<i32>().unwrap();
                    over_threadhold_server.hostname = hostname::get().unwrap().into_string().unwrap();
                    over_threadhold_server.filesystem = line[0].to_string();
                }
            }
            over_threadhold_server
        }
    }
}

#[cfg(test)]
mod tests {
    use super::my_server_disk::DiskInfo;
 
    #[test]
    fn test_checkout_local_storage_over_90_percent() {
        let mut data = DiskInfo::get_server_data();
        data.total = 100;
        data.used = 91;
        data.free = 9;
        data.percent = 91.0;
        data.mount_on = "/".to_string();
        data.disk_name = "/dev/sda1".to_string();
        assert_eq!(data.percent > 90.0, true);
    }

    #[test]
    fn test_checkout_local_storage_less_90_percent() {
        let mut data = DiskInfo::get_server_data();
        data.total = 100;
        data.used = 89;
        data.free = 11;
        data.percent = 89.0;
        data.mount_on = "/".to_string();
        data.disk_name = "/dev/sda1".to_string();
        assert_eq!(data.percent > 90.0, false);
    }
}
