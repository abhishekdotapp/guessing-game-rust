use std::io;
use rand::prelude::*;
fn main() {
    let arrlist = ["apple","banana","orange","mango"];
    //let num = 2;
   // println!("{}",num);
    let mut user = String::new();
    let mut rng = thread_rng();
    let index = rng.gen_range(0..arrlist.len());
    let rand_ans = arrlist[index];
    println!("{}",rand_ans);
    //let mut chances = 3;
    loop{
    
    match std::io::stdin().read_line(&mut user){
         Ok(_)=>{
            let fruit_name = user.trim().to_lowercase().to_string();
            /*if !arrlist.contains(&fruit_name.as_str()){
                println!("input is out of range");
                continue;
            }*/
            match guess_match(&fruit_name,&rand_ans){

    true=>{

      println!("Correct Guess! You are a winner!");

      break;

    }false=>{

      println!("Retry");

    }
    
         }
         }
         Err(e)=>{
             println!("error ={}",e);
         }
    }
    
  }

  }
    
    


fn guess_match(a:&str,b:&str)->bool{
    a == b
}
