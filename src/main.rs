#[warn(unused_imports)]
use std::{env, fs::{self}};
use reqwest::{Result, Response};

#[tokio::main]
async fn main() {
    intro();
    let args: Vec<String> = env::args().collect();
    if !args_checker(&args){
        println!("Usage -w <path> -u <url>");
        return;
    }
    let file_name = &args[2];
    let url = &args[4];
    let file_f = fs::read_to_string(file_name).unwrap();
    let lines = file_f.lines();

    for line in lines  {
        let body = reqwest::get(url.to_owned()+line).await;
        get_response(body, &url, &line).await;
    }
}

fn args_checker(args : &Vec<String>)->bool{
    if args[1] == "-w" && args[3] == "-u" && args.len() == 5{
        println!("Path: {}", args[2]);
        println!("Url: {}", args[4]);
        println!("-----------------------------------------------------------------");
        return true;
    }
    return false;
}

async fn get_response(body: Result<Response>, url: &String, path: &str){
    match body{
        Ok(response) => {   
            print!("Url: {}{}\t",url,path);
            print!("Status: {:?}\t",response.status());
            println!("Size: {:?}",response.headers()["content-length"]);
        },
        Err(e) => {
            println!("Error: {}",e);
        }
    }
}

fn intro(){
    println!("
    ▄████████    ▄████████    ▄████████  ███    █▄     ▄███████▄ 
    ███    ███   ███    ███   ███    ███ ███    ███   ███    ███ 
    ███    ███   ███    █▀    ███    ███ ███    ███   ███    ███ 
   ▄███▄▄▄▄██▀  ▄███▄▄▄      ▄███▄▄▄▄██▀ ███    ███   ███    ███ 
  ▀▀███▀▀▀▀▀   ▀▀███▀▀▀     ▀▀███▀▀▀▀▀   ███    ███ ▀█████████▀  
  ▀███████████   ███    █▄  ▀███████████ ███    ███   ███        
    ███    ███   ███    ███   ███    ███ ███    ███   ███        
    ███    ███   ██████████   ███    ███ ████████▀   ▄████▀      
    ███    ███                ███    ███                         
    \n\t\t\tMade by ptukovar\n");
}
