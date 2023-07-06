#[warn(unused_imports)]
use std::{env, fs::{self}, io::Write};
use reqwest::{Result, Response, StatusCode};
use std::fs::OpenOptions;

#[tokio::main]
async fn main() {
    intro();
    let args: Vec<String> = env::args().collect();
    if !args_checker(&args){
        println!("Usage -w <path> -u <url> -o <output_file>");
        return;
    }
    let file_name = &args[2];
    let url = &args[4];
    
    if !url.contains("FUZZ"){
        println!("Missing FUZZ text in url");
        return;
    }
    let file_f = fs::read_to_string(file_name).unwrap();
    let lines = file_f.lines();

    for line in lines  {
        let link = url_formater(url, &line.to_owned());
        let body = reqwest::get(&link).await;
        if args[5]=="-o"{
            let outname = args[6].to_string();
            get_response(body, &link, outname).await
        };
    }
}

fn args_checker(args : &Vec<String>)->bool{
    if args[1] == "-w" && args[3] == "-u" && args.len() == 5{
        println!("Path: {}", args[2]);
        println!("Url: {}", args[4]);
        println!("-----------------------------------------------------------------");
        return true;
    }else if args[1] == "-w" && args[3] == "-u" && args[5]=="-o"{
        println!("Path: {}", args[2]);
        println!("Url: {}", args[4]);
        println!("Output: {}", args[6]);
        println!("-----------------------------------------------------------------");
        return true;
    }
    return false;
}

async fn get_response(body: Result<Response>, url: &String, outname : String){
    match body{
        Ok(response) => {   
            print!("Url: {}\t",url);
            print!("Status: {:?}\t",response.status());
            println!("Size: {:?}",response.headers()["content-length"]);
            let size = format!("{:?}",response.headers()["content-length"]);
            save_f(url, response.status(), size, outname);
        },
        Err(e) => {
            println!("Error: {}",e);
        }
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
    return result;
}
fn save_f(url : &String, status : StatusCode, size: String, outname: String){
    let buf = format!("Url: {}\tStatus: {:?}\tSize: {:?}\n",url,status,size);
    let mut f = OpenOptions::new().create(true).append(true).open(outname).expect("Error");
    f.write(buf.as_bytes()).expect("Error");
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
