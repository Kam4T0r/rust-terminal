// drogi programisto
// tylko ja i Bóg wiedzieliśmy jak działa ten kod
// teraz tylko Bóg wie
// jebać debiana arch najlepszy
use std::process::{Command, exit};
use std::{fs, process, thread};
use std::env::{args, current_dir, current_exe};
use std::fs::{create_dir, read_dir, rename};
use std::io::{BufRead, Read, stdout, Write};
use std::iter::StepBy;
use std::path::{Path, PathBuf};
use std::thread::current;
use std::time::{Duration, SystemTime};
extern crate chrono;
use chrono::prelude::*;
extern crate whoami;
use whoami::{fallible, platform};

fn main(){
    loop {
        print!(">");
        let _ = stdout().flush();
        let mut quest = String::new();
        std::io::stdin().read_line(&mut quest).unwrap();

        match quest.trim() {
            "mdir" | "mkdir" | "ndir" | "newdir" =>{
                println!("enter name of the folder");
                print!(">");
                let _ = stdout().flush();
                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                println!("enter number of folders"); // to dla Pana Pietrzaka
                print!(">");
                let _ = stdout().flush();

                let mut quantity = String::new();
                std::io::stdin().read_line(&mut quantity).expect("error");
                let quantity: i32 = quantity.trim().parse().expect("error");

                if quest.trim() != ""{
                    if quantity == 1{
                        fs::create_dir_all(args.trim()).unwrap();
                    }
                    else {
                        for i in 1..quantity + 1 {
                            let path: String = args.trim().to_owned() + &i.to_string();
                            fs::create_dir_all(path).unwrap();
                        }
                    }
                }
                else {
                    println!("error! you can't leave this empty!");
                }
            },
            "nfile" | "touch" | "newfile" =>{
                println!("enter name of the file");
                print!(">");
                let _ = stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                println!("enter extension of the file e.g. txt or exe");
                print!(">");
                let _ = stdout().flush();

                let mut extension = String::new();
                std::io::stdin().read_line(&mut extension).unwrap();

                println!("enter number of files");
                print!(">");
                let _ = stdout().flush();

                let mut quantity = String::new();
                std::io::stdin().read_line(&mut quantity).expect("error");
                let quantity: i32 = quantity.trim().parse().expect("error");

                if args.trim() != "" {
                    if quantity == 1 {
                        let path = args.trim().to_owned() + stringify!(.) + extension.trim();
                        fs::File::create_new(path).unwrap();
                    }
                    else {
                        for i in 1..quantity + 1 {
                            let path = args.trim().to_owned() + &i.to_string() + stringify!(.) + extension.trim();
                            fs::File::create_new(path).unwrap();
                        }
                    }
                }
                else {
                    println!("error! you can't leave name empty!")
                }
            },
            "rm" | "remove" | "rmfile" | "rfile" =>{
                println!("which file do you want to remove?");

                print!(">");
                let _ = stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                if args.trim() != "" {
                    fs::remove_file(args.trim()).unwrap();
                }
                else {
                    println!("invalid option! you can't leave this empty!");
                }
            },
            "rdir" | "rmdir" | "removedir" | "removedirectory" =>{
                println!("enter directory you want to delete (with all of containing files)");
                print!(">");
                let _ = stdout().flush();
                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                if args.trim() == "/" || args.trim() == "*" {
                    println!("are you sure you want to do this? executing this command will erase all data on disk yes/no ");
                    print!(">");
                    let _ = stdout().flush();

                    let mut quest = String::new();
                    std::io::stdin().read_line(&mut quest).unwrap();
                    let quest = quest.trim().to_uppercase();

                    if quest == "YES" {
                        fs::remove_dir_all(args.trim()).unwrap(); //usuwa / albo C:\ więc lepiej wara od tego
                    }
                    else {
                        println!("aborting thread");
                    }
                }
                else if !(args.trim() == "") {
                    fs::remove_dir_all(args.trim()).unwrap();
                }
                else {
                    println!("error! you can't leave this blank")
                }
            },
            "cd" | "goto" =>{
                println!("enter target directory");
                print!(">");
                let _ = stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                if !(args.trim() == ""){
                    std::env::set_current_dir(args.trim()).unwrap();
                }
                else {
                    println!("error! you can't leave this blank!");
                }
            },
            "pwd" | "here" | "whereami" =>{
              let current_dir = std::env::current_dir();
              println!("working direcory is {} ", current_dir.unwrap().display());
            },
            "list" | "ls" | "listfiles" | "lsdir" | "listdirectory" =>{
                println!("which directory do you want to list? (leave blank for current)");
                print!(">");
                let _ = stdout().flush();

                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();

                if args.trim() == "" {
                    let path = fs::read_dir(std::env::current_dir().unwrap()).unwrap();
                    for files in path{
                        println!("Name: {}", files.unwrap().path().display());
                    }
                }
                else {
                    let path = fs::read_dir(args.trim()).unwrap();
                    for files in path{
                      println!("name: {}", files.unwrap().path().display());
                  }
                }
            },
            "fread" | "read"=>{
                println!("enter file name");
                print!(">");
                let _ = stdout().flush();

                let mut path = String::new();
                std::io::stdin().read_line(&mut path).expect("error");
                let path = path.trim();

                match fs::read_to_string(path){
                    Ok(contents) => println!("file {} contains:\n{}", path, contents),
                    Err(e) => println!("error reading file {}: {}", path, e),
                }
            },
            "rname" | "rename" =>{
                println!("enter filename");
                print!(">");
                let _ = stdout().flush();

                let mut args1 = String::new();
                std::io::stdin().read_line(&mut args1).expect("error");
                let args1 = args1.trim();

                println!("enter target filename");
                print!(">");
                let _ = stdout().flush();

                let mut args2 = String::new();
                std::io::stdin().read_line(&mut args2).expect("error");
                let args2 = args2.trim();

                let _ = fs::rename(args1, args2);
            },
            "whoami" | "info" | "who" =>{
                println!("here is your hardware and software info:");
                thread::sleep(Duration::from_millis(150));
                println!("username: {}",whoami::username());
                thread::sleep(Duration::from_millis(50));
                println!("user's real name: {}", whoami::realname()); // jak chinole szpiegujemy ludzi
                thread::sleep(Duration::from_millis(50));
                // println!("language: {:?}",whoami::langs().collect::<Vec<String>>()); //ogl jebać vektory
                // thread::sleep(Duration::from_millis(50));
                println!("device's name: {}",whoami::devicename());
                thread::sleep(Duration::from_millis(50));
                match whoami::fallible::hostname(){
                    Ok(hostname) => println!("device's hostname {}",hostname),
                    Err(error) => println!("error while reading hostname: {}",error)
                }
                thread::sleep(Duration::from_millis(50));
                println!("device's platform: {}",whoami::platform());
                thread::sleep(Duration::from_millis(50));
                println!("device's distro: {}",whoami::distro());
                thread::sleep(Duration::from_millis(50));
                println!("device's desktop environment: {}",whoami::desktop_env());
                thread::sleep(Duration::from_millis(50));
                println!("CPU architecture: {}",whoami::arch());
            }
            "time" | "clock" =>{
                let current_time: DateTime<Local> = Local::now(); // chuj wie jak działa ale działa więc nie dotykać tego
                println!("current time is {:?}", current_time);
            },
            "leave" | "exit" | "close" =>{
                process::exit(0);
            },
            "clear" | "cls" =>{
                clear_screen();
            },
            "help" =>{
              println!(" leave/exit/close - exit terminal\n clear/cls - clear screen\n mkdir/mdir/ndir/newdir - creates new empty directory\n nfile/newfile/touch - make file\n rm/remove/rmfile/rfile - remove single file\n rdir/rmdir/removedir/removedirectory - removes directory with all containing files\n cd/goto - change directory\n pwd/whereami/here - show working directory\n list/ls/listfiles/lsdir/listdirectory - shows files in directory\n time/clock - displays current time\n fread/read - prints file contains\n rename/rname - renames file/folder\n whoami/info/who - displays info about hardware and software");
            },
            _ => {
                println!("invalid option! type 'help' for full command list");
            }
        }
    }
}

fn clear_screen(){
    if cfg!(target_os = "windows"){
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    }
    else{
        Command::new("clear").status().unwrap();
    }
}
