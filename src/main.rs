#[warn(unused_imports)]
use std::{env, fs::{self}, io::Write};
use reqwest::{Result, Response};
use std::fs::OpenOptions;
use colored::Colorize;
use std::sync::{Arc, Mutex};

#[derive(Clone)]    
struct Resp {
    ur: String,
    stat: String,
    size: String,
}

#[tokio::main]
async fn main() {
    intro();
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 2 && (args[1] == "-h" || args[1] == "-help") {
        help();
        return;
    } else {
        if args.len() < 12 {
            while args.len() < 12 {
                args.push("".to_string());
            }
        }

        args = tags_checker(&args);
        if args.is_empty() {
            return;
        } else {
            args_checker(&args);
        }
    }
    let file_name = &args[2];
    let url = &args[4];

    if !url.contains("FUZZ") {
        println!("Missing FUZZ text in url");
        return;
    }

    let file_f = fs::read_to_string(file_name).unwrap();
    let lines = file_f.lines();

    let mut tasks = vec![];
    let resp: Arc<Mutex<Vec<Resp>>> = Arc::new(Mutex::new(Vec::new()));

    let link_arc = Arc::new(url.to_owned());
    let args_arc = Arc::new(args.clone());

    if args[10] == "" {
        for line in lines {
            let link = url_formater(url, &line.to_owned());
            let link_clone = link_arc.clone();
            let args_clone = args_arc.clone();
            let body = reqwest::get(&link).await;
            if args[5] == "-o" {
                let outname = args[6].to_string();
                let resp_clone = resp.clone();
                tasks.push(tokio::spawn(async move {
                    get_response(body, &link_clone, outname, resp_clone, &args_clone).await
                }));
            } else {
                let outname = " ".to_string();
                let resp_clone = resp.clone();
                tasks.push(tokio::spawn(async move {
                    get_response(body, &link_clone, outname, resp_clone, &args_clone).await
                }));
            }
        }
    } else if args[10] != "" {
        let extensions: Vec<&str> = args[10].split(',').collect();
        for line in lines {
            for ext in &extensions {
                let link = url_formater_ext(url, &line.to_owned(), &ext);
                let link_clone = link_arc.clone();
                let args_clone = args_arc.clone();
                let body = reqwest::get(&link).await;
                if args[5] == "-o" {
                    let outname = args[6].to_string();
                    let resp_clone = resp.clone();
                    tasks.push(tokio::spawn(async move {
                        get_response(body, &link_clone, outname, resp_clone, &args_clone).await
                    }));
                } else {
                    let outname = " ".to_string();
                    let resp_clone = resp.clone();
                    tasks.push(tokio::spawn(async move {
                        get_response(body, &link_clone, outname, resp_clone, &args_clone).await
                    }));
                }
            }
        }
    }
}


fn tags_checker(args: &Vec<String>)->Vec<String>{
    let mut sorted_args: Vec<String> = args.to_owned();

    let mut i;
    for (index, arg) in args.iter().enumerate() {
        if arg.contains("-w"){
            i=1;
            sorted_args[i]=arg.to_string();
            sorted_args[i+1]=args[index+1].to_string();
        }else if arg.contains("-u"){
            i=3;
            sorted_args[i]=arg.to_string();
            sorted_args[i+1]=args[index+1].to_string();
        }else if arg.contains("-o"){
            i=5;
            sorted_args[i]=arg.to_string();
            sorted_args[i+1]=args[index+1].to_string();
        }else if arg.contains("-si"){
            i=7;
            sorted_args[i]=arg.to_string();
            sorted_args[i+1]=args[index+1].to_string();
        }else if arg.contains("-st"){
            i=7;
            sorted_args[i]=arg.to_string();
            sorted_args[i+1]=args[index+1].to_string();
        }else if arg.contains("-x"){
            i=9;
            sorted_args[i]=arg.to_string();
            sorted_args[i+1]=args[index+1].to_string();
        }
    }
    
    if sorted_args.contains(&"-w".to_string()){
        if sorted_args.contains(&"-u".to_string()) {
            return sorted_args;   
        }else{
            return Vec::new();
        }
    } else{
        println!("Usage -w <path> -u <url> -o <output_file>");
        return Vec::new();
    }
}


fn args_checker(args : &Vec<String>)->bool{
    if args[1] == "-w" && args[3] == "-u" && args.len() == 5{
        println!("Path: {}", args[2]);
        println!("Url: {}", args[4]);
    
        for i in 0..65{
            if i % 2 == 0 {
                print!("{}", "─".white());
            } else {
                print!("{}", "─".black());
            }
        }
        println!();
        return true;
    }else if args[1] == "-w" && args[3] == "-u" && args[5]=="-o"{
        println!("Path: {}", args[2]);
        println!("Url: {}", args[4]);
        println!("Output: {}", args[6]);
        for i in 0..65{
            if i % 2 == 0 {
                print!("{}", "─".white());
            } else {
                print!("{}", "─".black());
            }
        }
        println!();
        return true;
    }
    return false;
}


async fn get_response(body: Result<Response>, url: &String, outname: String, resp: Arc<Mutex<Vec<Resp>>>, args: &Vec<String>) -> Vec<Resp> {
    let mut responses = Vec::new();
    match body {
        Ok(response) => {
            let path = response.url().path().replace("/", "");
            let full_url = url.replace("FUZZ", &path);
            let ur = format!("{}", full_url);
            let stat = format!("{:?}", response.status());
            let size = format!("{:?}", response.headers()["content-length"]).replace('"', "");
            let res = Resp { ur, stat, size };

            let mut fi = false;
            for x in args {
                if x.contains("-st") || x.contains("-si") {
                    fi = true;
                }
            }

            if fi == true {
                let filter = &args[8];
                if args[7] == "-st" {
                    if args[8].starts_with("=") {
                        let filter_values: Vec<&str> = filter[1..].split(',').collect();
                        for x in filter_values {
                            if &res.stat.parse::<i32>().unwrap() == &x.parse::<i32>().unwrap() {
                                response_printer(&res);
                                save_f(&res.ur, &res.stat, &res.size, &outname);
                                let mut locked_resp = resp.lock().unwrap();
                                locked_resp.push(res.to_owned());
                                responses.push(res.to_owned()); 
                            }
                        }
                    } else if args[8].starts_with("!=") {
                        let filter_values: Vec<&str> = filter[2..].split(',').collect();
                        for x in filter_values {
                            if &res.stat.parse::<i32>().unwrap() != &x.parse::<i32>().unwrap() {
                                response_printer(&res);
                                save_f(&res.ur, &res.stat, &res.size, &outname);
                                let mut locked_resp = resp.lock().unwrap();
                                locked_resp.push(res.to_owned());
                                responses.push(res.to_owned());
                            }
                        }
                    } else if args[8].starts_with(">") {
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.stat.parse::<i32>().unwrap() > filter_val {
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            let mut locked_resp = resp.lock().unwrap();
                            locked_resp.push(res.to_owned());
                            responses.push(res.to_owned());
                        }
                    } else if args[8].starts_with("<") {
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.stat.parse::<i32>().unwrap() < filter_val {
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            let mut locked_resp = resp.lock().unwrap();
                            locked_resp.push(res.to_owned());
                            responses.push(res.to_owned());
                        }
                    }
                } else if args[7] == "-si" {
                    if args[8].starts_with("=") {
                        let filter_values: Vec<&str> = filter[1..].split(',').collect();
                        for x in filter_values {
                            if &res.size.parse::<i32>().unwrap() == &x.parse::<i32>().unwrap() {
                                response_printer(&res);
                                save_f(&res.ur, &res.stat, &res.size, &outname);
                                let mut locked_resp = resp.lock().unwrap();
                                locked_resp.push(res.to_owned());
                                responses.push(res.to_owned());
                            }
                        }
                    } else if args[8].starts_with("!=") {
                        let filter_values: Vec<&str> = filter[2..].split(',').collect();
                        for x in filter_values {
                            if &res.size.parse::<i32>().unwrap() != &x.parse::<i32>().unwrap() {
                                response_printer(&res);
                                save_f(&res.ur, &res.stat, &res.size, &outname);
                                let mut locked_resp = resp.lock().unwrap();
                                locked_resp.push(res.to_owned());
                                responses.push(res.to_owned());
                            }
                        }
                    } else if args[8].starts_with(">") {
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.size.parse::<i32>().unwrap() > filter_val {
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            let mut locked_resp = resp.lock().unwrap();
                            locked_resp.push(res.to_owned());
                            responses.push(res.to_owned());
                        }
                    } else if args[8].starts_with("<") {
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.size.parse::<i32>().unwrap() < filter_val {
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            let mut locked_resp = resp.lock().unwrap();
                            locked_resp.push(res.to_owned());
                            responses.push(res.to_owned());
                        }
                    }
                }
            } else {
                response_printer(&res);
                if args[6] == "-o" {
                    save_f(&res.ur, &res.stat, &res.size, &outname);
                    let mut locked_resp = resp.lock().unwrap();
                    locked_resp.push(res.to_owned());
                    responses.push(res.to_owned());
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    responses
}

fn response_printer(res : &Resp){
    if res.stat == "404" {
        print!("Url: {}\t",res.ur.red());
        print!("Status: {}\t",res.stat.red());
        println!("Size: {}",res.size.red()); 
    }else if res.stat == "200" || res.stat == "202"{
        print!("Url: {}\t",res.ur.bright_green());
        print!("Status: {}\t",res.stat.bright_green());
        println!("Size: {}",res.size.bright_green());
    }else{
        print!("Url: {}\t",res.ur.blue());
        print!("Status: {}\t",res.stat.blue());
        println!("Size: {}",res.size.blue());
    }
    
}

fn url_formater(url : &String, line : &String)->String{
    let partes: Vec<&str> = url.split("FUZZ").collect();
    let result;
    if partes.len()>=2 {
        result=partes[0].to_owned()+line+partes[1];
    }else{
        result=partes[0].to_owned()+line;
    }
    return result
}

fn url_formater_ext(url : &String, line : &String, ext : &str)->String{
    let partes: Vec<&str> = url.split("FUZZ").collect();
    let result;
    if partes.len()>=2 {
        result=partes[0].to_owned()+line+ext+partes[1];
    }else{
        result=partes[0].to_owned()+line+ext;
    }
    return result
}

fn save_f(url : &String, status : &String, size: &String, outname: &String){
    if outname!=" "{
        let buf = format!("Url: {}\tStatus: {}\tSize: {}\n",url,status,size);
        let mut f = OpenOptions::new().create(true).append(true).open(outname).expect("Error");
        f.write(buf.as_bytes()).expect("Error");
    }
}


    
fn intro() {
    let text = "
    ▄████████    ▄████████    ▄████████  ███    █▄     ▄███████▄
    ███    ███   ███    ███   ███    ███ ███    ███   ███    ███
    ███    ███   ███    █▀    ███    ███ ███    ███   ███    ███
   ▄███▄▄▄▄██▀  ▄███▄▄▄      ▄███▄▄▄▄██▀ ███    ███   ███    ███
  ▀▀███▀▀▀▀▀   ▀▀███▀▀▀     ▀▀███▀▀▀▀▀   ███    ███ ▀█████████▀
  ▀███████████   ███    █▄  ▀███████████ ███    ███   ███
    ███    ███   ███    ███   ███    ███ ███    ███   ███
    ███    ███   ██████████   ███    ███ ████████▀   ▄████▀
    ███    ███                ███    ███
    ";
    let colored_text = text
        .replace("▄", &"▄".black().to_string())
        .replace("█", &"█".red().to_string())
        .replace("▀", &"▀".red().to_string());

    println!("{}", colored_text);
    println!("{}{}","\n\t\t\tMade by ", "ptukovar\n".blue());
}

fn help() {
    println!("Usage: -w <path> -u <url> -o <output_file>");
    println!("Options:");
    println!("-h, -help\tDisplay this help message [--]");
    println!("-w\t\tSpecify the input file path [--]");
    println!("-u\t\tSpecify the URL with 'FUZZ' as a placeholder");
    println!("-o\t\tSpecify the output file path");
    println!("-st\t\tFilter by status code (e.g., -st =200)");
    println!("-si\t\tFilter by response size (e.g., -si >1000)");
    println!("Example: rerup -w inputs.txt -u http://127.0.0.1:8000/FUZZ -o output.txt -st =200");
}
