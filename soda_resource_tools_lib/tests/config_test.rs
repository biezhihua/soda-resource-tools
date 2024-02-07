#[cfg(test)]
mod config_tests {
    use std::{
        fs::File,
        io::{Read, Write},
    };

    use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    use serde_json::{json, Value};

    // #[test]
    fn gen_bin() {
        let current_path = std::env::current_dir().unwrap();
        let strong_match_rules_tv_path = current_path
            .join("config")
            .join("mt_strong_match_rules_tv.json")
            .to_str()
            .unwrap()
            .to_string();
        let strong_match_rules_movie_path = current_path
            .join("config")
            .join("mt_strong_match_rules_movie.json")
            .to_str()
            .unwrap()
            .to_string();
        let strong_match_regex_rules_path = current_path
            .join("config")
            .join("mt_strong_match_regex_rules.json")
            .to_str()
            .unwrap()
            .to_string();
        let strong_match_name_map_path = current_path
            .join("config")
            .join("mt_strong_match_name_map.json")
            .to_str()
            .unwrap()
            .to_string();

        // 合并 JSON 数据
        let mut combined_json = json!({});

        // tv
        {
            let mut file = File::open(strong_match_rules_tv_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut value: Value = serde_json::from_str(&contents).unwrap();
            let rules = value.get_mut("rules").unwrap().as_array_mut().unwrap();
            for ele in rules {
                ele["example"] = Value::String("".to_string());
            }
            combined_json["mt_strong_match_rules_tv"] = value;
        }

        // movie
        {
            let mut file = File::open(strong_match_rules_movie_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut value: Value = serde_json::from_str(&contents).unwrap();
            let rules = value.get_mut("rules").unwrap().as_array_mut().unwrap();
            for ele in rules {
                ele["example"] = Value::String("".to_string());
            }
            combined_json["mt_strong_match_rules_movie"] = value;
        }

        // rules
        {
            let mut file = File::open(strong_match_regex_rules_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let value: Value = serde_json::from_str(&contents).unwrap();
            combined_json["mt_strong_match_regex_rules"] = value;
        }

        // map
        {
            let mut file = File::open(strong_match_name_map_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let mut value: Value = serde_json::from_str(&contents).unwrap();
            let names = value.get_mut("names").unwrap().as_array_mut().unwrap();
            for ele in names {
                ele["example"] = Value::String("".to_string());
            }
            combined_json["mt_strong_match_name_map"] = value;
        }

        let mut bin_out_file = File::create(
            current_path
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("soda_cl")
                .join("soda_config_rule.json"),
        )
        .unwrap();

        let combined_str = serde_json::to_string(&combined_json).unwrap();

        // let mc = new_magic_crypt!("biezhihua_soda", 256);

        // let encoded = mc.encrypt_str_to_base64(combined_str);

        bin_out_file.write_all(combined_str.as_bytes()).unwrap();

        let soda_config_json = json!({
            "version": 0,
            "bin": "soda_config_rule.json",
            "enable_cli": true,
        });

        let mut json_out_file = File::create(current_path.parent().unwrap().parent().unwrap().join("soda_cl").join("soda_config.json")).unwrap();
        let combined_str = serde_json::to_string(&soda_config_json).unwrap();
        json_out_file.write_all(combined_str.as_bytes()).unwrap();
    }
}
