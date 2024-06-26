use std::fs;
use std::path::Path;
use std::process::Command;

fn compile(path: &Path) {

    if path.is_dir() {

        for child_dir in fs::read_dir(path).unwrap() {
            
            for file in fs::read_dir(child_dir.unwrap().path()).unwrap() {

                let file_path = file.unwrap().path();

                let mut path_index = file_path.to_str().unwrap().split("/").collect::<Vec<&str>>();
                path_index.pop().unwrap();

                let cur_dir = path_index.join("/");

                if file_path.to_str().unwrap().contains("INFO") {

                    let info_cxt = fs::read_to_string(file_path).unwrap();
                    let lang = info_cxt.trim().split("\n").into_iter().last().unwrap();

                    match lang.to_uppercase().as_str() {

                        "RUST" => {
                            Command::new("rustc").
                                current_dir(cur_dir).
                                arg("main.rs").
                                spawn().
                                unwrap();
                        }
                        
                        "C" => {
                            Command::new("gcc").
                                current_dir(cur_dir).
                                args(["main.c", "-o", "main"]).
                                spawn().
                                unwrap();
                        }

                        "CPP" => {
                            Command::new("g++").
                                current_dir(cur_dir).
                                args(["main.cpp", "-o", "main"]).
                                spawn().
                                unwrap();
                        }

                        _ => {   }

                    }

                }

            }

        }

    }

}

fn git_clone(path: &Path) {

    let string_repos = fs::read_to_string("/home/arhant/Sandbox/rusty/jit_bot/src/git_repo_list").unwrap();
    let mut list_of_repos = string_repos.split("\n").collect::<Vec<&str>>();
    list_of_repos.pop();

    for (num, url) in list_of_repos.iter().enumerate() {

        let mut parent_path = String::from(path.to_str().unwrap());
        parent_path.push_str(&num.to_string());

        let temp_path = Path::new(&parent_path);
        fs::create_dir(temp_path).unwrap();

        Command::new("git")
            .args(["clone", url, "."])
            .current_dir(temp_path)
            .spawn()
            .unwrap();

    }

}

fn get_output(path: &Path, info: &String, round: i32, prev: &String) -> String {
    
    let mut temp_path = path.to_str().unwrap().to_string();

    match info.as_str() {

        "RUST" | "C" | "CPP" | "C++" => {
        
            let _ = temp_path.push_str("/main");
        
            let mut cmd_out = Command::new(temp_path)
                                 .args([round.to_string(), prev.to_string()])
                                 .output().unwrap().stdout;
        
            if *cmd_out.last().unwrap() == 10 as u8 {
                cmd_out.pop();
            }
        
            let out = String::from_utf8(cmd_out).unwrap();
        
            return out;
        
        }

        "PYTHON" => {

            let mut cmd_out = Command::new("python")
                                 .current_dir(path)
                                 .args(["main.py".to_string(), round.to_string(), prev.to_string()])
                                 .output().unwrap().stdout;

            if *cmd_out.last().unwrap() == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        "JS" => {

            let mut cmd_out = Command::new("node")
                                 .current_dir(path)
                                 .args(["main.js".to_string(), round.to_string(), prev.to_string()])
                                 .output().unwrap().stdout;

            if *cmd_out.last().unwrap() == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        "JAVA" => {

            let mut cmd_out = Command::new("java")
                                 .current_dir(path)
                                 .args(["main.java".to_string(), round.to_string(), prev.to_string()])
                                 .output().unwrap().stdout;

            if *cmd_out.last().unwrap() == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        "ELIXIR" => {

            let mut cmd_out = Command::new("elixir")
                                 .current_dir(path)
                                 .args(["main.exs".to_string(), round.to_string(), prev.to_string()])
                                 .output().unwrap().stdout;

            if *cmd_out.last().unwrap() == 10 as u8 {
                cmd_out.pop();
            }

            let out = String::from_utf8(cmd_out).unwrap();

            return out;

        }

        _ => {  }
    }

    "NONE".to_string()

}

fn compete(path: &Path) {

    let mut a_out: String;
    let mut b_out: String;

    let mut a_out_prev: String;
    let mut b_out_prev: String;

    for (index_a, entry_a) in fs::read_dir(path).unwrap().enumerate() {

        let path_a = entry_a.unwrap().path();

        let mut info_a_path = path_a.as_path().to_str().unwrap().to_string();
        info_a_path.push_str("/INFO");

        let mut info_a_read = fs::read_to_string(&info_a_path).unwrap();

        info_a_read.pop();

        let info_a = info_a_read.split("\n").last().unwrap().to_string();
        let info_name_a_vec = info_a_read.split("\n").collect::<Vec<_>>();
        let info_name_a = info_name_a_vec.get(0).unwrap();

        for (index_b, entry_b) in fs::read_dir(path).unwrap().enumerate() {

            if index_a != index_b {

                let path_b = entry_b.unwrap().path();

                let mut info_b_path = path_b.as_path().to_str().unwrap().to_string();
                info_b_path.push_str("/INFO");

                let mut info_b_read = fs::read_to_string(&info_b_path).unwrap();
                info_b_read.pop();
                let info_b = info_b_read.split("\n").last().unwrap().to_string();

                let info_name_b_vec = info_b_read.split("\n").collect::<Vec<_>>();
                let info_name_b = info_name_b_vec.get(0).unwrap();

                for round in 1..=5 {

                    a_out = get_output(&path_a, &info_a.to_uppercase(), round, &"YES".to_string());
                    b_out = get_output(&path_b, &info_b.to_uppercase(), round, &"YES".to_string());

                    println!("ROUND BETWEEN {:?} {:?} AND OUTPUT {:?} {:?} ROUND NO {:?}",
                             info_name_a, 
                             info_name_b,
                             a_out,
                             b_out,
                             round);

                }

            }

        }

        println!("\n");

    }

}

fn main() {

    // git_clone(Path::new("/home/arhant/JIT_CUP/playground/"));
    // compile(Path::new("/home/arhant/JIT_CUP/playground/"));
    compete(Path::new("/home/arhant/JIT_CUP/playground/"));


}


/* [C, CPP, Rust, JS, Java Python] */
