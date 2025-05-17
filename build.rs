use std::process::Command;

fn main() {
    for version in ["2018c", "2018g", "2021b", "2023d"] {
        let tzdata_dir = format!("./external/{}", version);
        let output_dir = format!("../../input/zoneinfo/{}", version);

        let status = Command::new("zic")
            .args([
                "-d",
                &output_dir,
                "africa",
                "asia",
                "antarctica",
                "northamerica",
                "southamerica",
                "europe",
            ])
            .current_dir(tzdata_dir)
            .status()
            .expect("Failed to execute zic");

        if !status.success() {
            panic!("zic failed with status: {}", status);
        }
    }

    println!("cargo:rerun-if-changed=external/");
}
