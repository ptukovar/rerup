#[warn(unused_imports)]
use std::{env, fs::{self}, io::Write};
use reqwest::{Result, Response};
use std::fs::OpenOptions;
use colored::Colorize;

//-x exte php, txt, jpg
#[tokio::main]
async fn main() {
    intro();
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 2 && (args[1] == "-h" || args[1] == "-help") {
        help();
        return;
    }else{
        args=tags_checker(&args);
        if args.is_empty(){
            return;
        }else{
            args_checker(&args);
        }
    }
    let file_name = &args[2];
    let url = &args[4];
    
    if !url.contains("FUZZ"){
        println!("Missing FUZZ text in url");
        return;
    }
    let file_f = fs::read_to_string(file_name).unwrap();
    let lines = file_f.lines();
    let mut resp :Vec<Resp> = Vec::new();
    for line in lines  {
        let link = url_formater(url, &line.to_owned());
        let body = reqwest::get(&link).await;
        if args[5]=="-o"{
            let outname = args[6].to_string();
            get_response(body, &link, outname, &mut resp, &args).await;
        }
    }
}

fn tags_checker(args: &Vec<String>)->Vec<String>{
    let mut sorted_args: Vec<String> = args.to_owned();

    let mut i;
    for (index, arg) in args.iter().enumerate() {
        sorted_args.push("x".to_string());
        if arg.contains("-w"){
            i=1;
            sorted_args[i]=arg.to_string();
            i=i+1;
            sorted_args[i]=args[index+1].to_string();
        }else if arg.contains("-u"){
            i=3;
            sorted_args[i]=arg.to_string();
            i=i+1;
            sorted_args[i]=args[index+1].to_string();
        }else if arg.contains("-o"){
            i=5;
            sorted_args[i]=arg.to_string();
            i=i+1;
            sorted_args[i]=args[index+1].to_string();
        }else if arg.contains("-si"){
            i=7;
            sorted_args[i]=arg.to_string();
            i=i+1;
            sorted_args[i]=args[index+1].to_string();
        }else if arg.contains("-st"){
            i=7;
            sorted_args[i]=arg.to_string();
            i=i+1;
            sorted_args[i]=args[index+1].to_string();
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

async fn get_response(body: Result<Response>, url: &String, outname: String, resp: &mut Vec<Resp>, args: &Vec<String>) -> Vec<Resp> {
    match body{
        Ok(response) => {
            let ur = format!("{}",url.to_string());
            let stat = format!("{:?}",response.status());
            let size = format!("{:?}",response.headers()["content-length"]).replace('"', "");
            let res = &Resp{ur,stat,size};

            if args.len()==9 {
                let filter = &args[8];
                if args[7]=="-st"{    
                    if args[8].starts_with("="){
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.stat.parse::<i32>().unwrap()==filter_val{
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            resp.push(res.to_owned());
                        }
                    }if args[8].starts_with("!="){
                        let filter_val = &filter[2..].parse::<i32>().unwrap();
                        if &res.stat.parse::<i32>().unwrap()!=filter_val{
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            resp.push(res.to_owned());
                        }
                    }if args[8].starts_with(">"){
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.stat.parse::<i32>().unwrap()>filter_val{
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            resp.push(res.to_owned());
                        }
                    }if args[8].starts_with("<"){
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.stat.parse::<i32>().unwrap()<filter_val{
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            resp.push(res.to_owned());
                        }
                    }
                }else if args[7]=="-si" {
                    if args[8].starts_with("="){
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.size.parse::<i32>().unwrap()==filter_val{
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            resp.push(res.to_owned());
                        }
                    }if args[8].starts_with("!="){
                        let filter_val = &filter[2..].parse::<i32>().unwrap();
                        if &res.size.parse::<i32>().unwrap()!=filter_val{
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            resp.push(res.to_owned());
                        }
                    }if args[8].starts_with(">"){
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.size.parse::<i32>().unwrap()>filter_val{
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            resp.push(res.to_owned());
                        }
                    }if args[8].starts_with("<"){
                        let filter_val = &filter[1..].parse::<i32>().unwrap();
                        if &res.size.parse::<i32>().unwrap()<filter_val{
                            response_printer(&res);
                            save_f(&res.ur, &res.stat, &res.size, &outname);
                            resp.push(res.to_owned());
                        }
                    }
                }
            }else{
                response_printer(&res);
                save_f(&res.ur, &res.stat, &res.size, &outname);
                resp.push(res.to_owned());
            }
            return resp.to_vec();
        },
        Err(e) => {
            println!("Error: {}",e);
            return resp.to_vec();
        }
    }
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

fn save_f(url : &String, status : &String, size: &String, outname: &String){
    let buf = format!("Url: {}\tStatus: {}\tSize: {}\n",url,status,size);
    let mut f = OpenOptions::new().create(true).append(true).open(outname).expect("Error");
    f.write(buf.as_bytes()).expect("Error");
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

#[derive(Clone)]
struct Resp{
    ur: String ,
    stat: String,
    size: String,
}
