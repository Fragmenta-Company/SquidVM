use crate::errdef::UPDATE_CHECK_ERR;
use std::process;

/// Gets update and returns Vector containing all strings necessary for displaying
/// new versions, return all necessary information.
#[cfg(not(test))]
pub fn get_update() -> Vec<String> {
    let mut mainvec = Vec::new();
    let response =
        match minreq::get("https://api.github.com/repos/Fragmenta-Company/SquidVM/releases/latest")
            .with_header("User-Agent", "SquidVM")
            .with_header("Accept", "application/vnd.github.v3+json")
            .send()
        {
            Ok(response) => response,
            Err(err) => {
                eprintln!("\x1B[41mError while looking for updates: {err}\x1b[0m");
                process::exit(UPDATE_CHECK_ERR);
            }
        };

    let json = match response.json::<serde_json::Value>() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("\x1B[41mError while looking for updates: {err}\x1b[0m");
            process::exit(UPDATE_CHECK_ERR);
        }
    };
    let tagname = json["tag_name"].as_str();
    let parsed_data = if let Some(latest_version) = tagname {
        latest_version
    } else {
        eprintln!("\x1B[41mError while looking for updates: Tag name is inexistent!\x1b[0m");
        process::exit(UPDATE_CHECK_ERR);
    };

    let mut latest_version: String = String::from(parsed_data);

    latest_version = String::from(latest_version.trim_start_matches("V"));

    let latest_version: Vec<&str> = latest_version.split(['.', '-']).collect();

    let current_version = String::from(env!("CARGO_PKG_VERSION"));

    let current_version: Vec<&str> = current_version.split(['.', '-']).collect();

    let majors: [u32; 2] = [
        latest_version[0].parse().unwrap(),
        current_version[0].parse().unwrap(),
    ];
    let minors: [u32; 2] = [
        latest_version[1].parse().unwrap(),
        current_version[1].parse().unwrap(),
    ];
    let patchs: [u32; 2] = [
        latest_version[2].parse().unwrap(),
        current_version[2].parse().unwrap(),
    ];

    let mut new_ver = 0;
    let mut versioning: Vec<bool> = Vec::new();

    if patchs[0] > patchs[1] && minors[0] >= minors[1] && majors[0] >= majors[1] {
        new_ver = 3;
        versioning.push(true);
    } else {
        versioning.push(false);
    }

    if minors[0] > minors[1] && majors[0] >= majors[1] {
        new_ver = 2;
        new_ver = 2;
        versioning.push(true);
    } else {
        versioning.push(false);
    }

    if majors[0] > majors[1] {
        new_ver = 1;
        versioning.push(true);
    } else {
        versioning.push(false);
    }

    let new = if new_ver == 1 {
        "New major!"
    } else if new_ver == 2 {
        "New minor!"
    } else if new_ver == 3 {
        "New patch!"
    } else {
        "No new version!"
    };

    let latest_ver_greater_3 = latest_version.len() > 3;
    let mut details_different = false;

    if latest_ver_greater_3 && current_version.len() > 3 {
        if latest_version[3] != current_version[3] {
            details_different = true;
        }
    } else if current_version.len() > 3 && !latest_ver_greater_3 {
        details_different = true;
    }

    let has_new_ver = new_ver != 0;

    if has_new_ver || details_different && !has_new_ver {
        let newer: Result<&str, &'static str> = if latest_ver_greater_3 {
            match latest_version[3] {
                "alpha" => Ok("New version is available."),
                "beta" => Ok("Project is now in beta!"),
                &_ => {
                    if latest_version[3].contains("rc") {
                        Ok("New Release Candidate!")
                    } else {
                        Err("Unidentified versioning on repository!")
                    }
                }
            }
        } else {
            Ok("Project is now released!")
        };
        let newer = newer.map_err(|err| format!("\x1B[41m{err}\x1B[0m"));

        if let Ok(newer) = newer {
            mainvec.push(format!("{newer}"));
        } else if let Err(err) = newer {
            mainvec.push(format!("\x1B[41m{err}\x1b[0m"));
        }
    }

    if has_new_ver || details_different {
        mainvec.push(format!("{new}"));
        let mut setter: Vec<&str> = Vec::new();
        if versioning[2] {
            versioning[1] = true;
            versioning[0] = true;
        } else if versioning[1] {
            versioning[0] = true;
        }
        versioning.iter().rev().for_each(|obj| {
            if obj == &true {
                setter.push("\x1B[32m")
            } else {
                setter.push("")
            }
        });

        if details_different {
            setter.push("\x1B[32m")
        } else {
            setter.push("")
        }

        mainvec.push(if latest_ver_greater_3 {
            format!(
                "Latest version: {}{}\x1b[0m.{}{}\x1b[0m.{}{}\x1b[0m-{}{}\x1b[0m",
                setter[0],
                latest_version[0],
                setter[1],
                latest_version[1],
                setter[2],
                latest_version[2],
                setter[3],
                latest_version[3]
            )
        } else {
            format!(
                "Latest version: {}{}\x1b[0m.{}{}\x1b[0m.{}{}\x1b[0m",
                setter[0],
                latest_version[0],
                setter[1],
                latest_version[1],
                setter[2],
                latest_version[2]
            )
        });
    } else {
        mainvec.push(if latest_ver_greater_3 {
            format!(
                "Latest version: {}.{}.{}-{}",
                latest_version[0], latest_version[1], latest_version[2], latest_version[3]
            )
        } else {
            format!(
                "Latest version: {}.{}.{}",
                latest_version[0], latest_version[1], latest_version[2]
            )
        });
    }

    mainvec
}

/// Gets update and returns Vector containing all strings necessary for displaying
/// new versions, return all necessary information.
#[cfg(test)]
pub fn get_update() {
    let response =
        minreq::get("https://api.github.com/repos/Fragmenta-Company/SquidVM/releases/latest")
            .with_header("User-Agent", "SquidVM")
            .with_header("Accept", "application/vnd.github.v3+json")
            .send()
            .unwrap();

    let json = response.json::<serde_json::Value>().unwrap();
    let tagname = json["tag_name"].as_str();
    let parsed_data = tagname.unwrap();

    let mut latest_version: String = String::from(parsed_data);

    latest_version = String::from(latest_version.trim_start_matches("V"));

    latest_version = String::from("10000.10000.10000-rc1");

    let latest_version: Vec<&str> = latest_version.split(['.', '-']).collect();

    let current_version = String::from(env!("CARGO_PKG_VERSION"));

    let current_version: Vec<&str> = current_version.split(['.', '-']).collect();

    let majors: [u32; 2] = [
        latest_version[0].parse().unwrap(),
        current_version[0].parse().unwrap(),
    ];
    let minors: [u32; 2] = [
        latest_version[1].parse().unwrap(),
        current_version[1].parse().unwrap(),
    ];
    let patchs: [u32; 2] = [
        latest_version[2].parse().unwrap(),
        current_version[2].parse().unwrap(),
    ];

    assert_eq!(majors[0], 10000);
    assert_eq!(minors[0], 10000);
    assert_eq!(patchs[0], 10000);

}
