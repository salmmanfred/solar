use web_view::*;
use openfile;
mod html;
use std::time::Duration;
mod simloader;
pub mod obj;
use std::path::Path;
pub mod errorwin;
fn main(){
    // declares the gravity constant
    let mut grc = 0.00002;
    // declares some test planets
    let planet1: obj::obj = obj::obj{
        name: "planet1".to_string(),
        x: 500.0,
        y: 250.0,
        mass: 1.0,
        size: 10.0,
        velx: 3.1,
        vely: 0.0,
        bounce: 1.1,

    };

    let planet2: obj::obj = obj::obj{
        name: "planet2".to_string(),
        x: 500.0,
        y: 50.0,
        mass: 10.0,
        size: 10.0,
        velx: 4.0,
        vely: 0.0,
        bounce: 1.1,

    };
    
    let sun: obj::obj = obj::obj{
        name: "sun".to_string(),
        x: 500.0,
        y: 450.0,
        mass: 100000.0,
        size: 50.0,
        velx: 0.0,
        vely: 0.0,
        bounce: 1.1,

    };
    let mut first = true;
    // ties with the display text bool on the other side
    let mut cl = true;

    // loads the objects into a vector where all the planets and stuff will reside 
    let mut objs = vec!(planet1.clone(), sun.clone(),planet2.clone());
    
    web_view::builder()
    // creates a window
        .title("Solar|Alpha 2")
        .content(Content::Html(htmlloader()))
        .size(1400, 900)
        .resizable(true)
        .debug(false)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                // for exiting the program
                "exit" => {
                    webview.exit();
                }
                // for starting the loop
                "start" =>{
                    println!("Start");
                    webview.eval("start()").expect("start error");

                }
                // for clearing the objs vector
                "clear" =>{
                    //let fobjs: Vec<obj::obj> = Vec::new();
                    objs = Vec::new();
                }
                "cl" =>{
                    cl = !cl;
                    //println!("{}",cl)
                    webview.eval(&format!("upbutcl('{}')",cl))?;
                    
                }
                "list" =>{
                    /*
                    if !first{
                        for x in objs.iter(){
                            webview.eval(&format!("clears('{}')",x.name))?;
                        }
                    }
                    for x in objs.iter(){
                    println!("addlist('{}',{},{})",x.name,x.x,x.y);

                        webview.eval(&format!("addlist('{}',{},{})",x.name,x.x,x.y)).expect("list error");
                    }
                    first = false;*/
                }
                // this is where the loop on the js side invokes too
                "run" =>{
                    //thread::sleep(Duration::from_millis(10));
                    //println!("xzczczxcxzczczxcxzcxz!{} y!{}",objs[0].x,objs[1].x);
                    // main for loop
                    for o in 0..objs.len(){
                        // secondary for loop
                        for xx in 0..objs.len(){
                            //println!("FFFFFFFFFFFFFFFFFx{} y{}",xx,o);
                            // makes sure its not calcuating in its own gravity
                            if xx != o{
                                // calls the gravity function
                                let modif = gravity(&objs[o], &objs[xx],grc);
                                // applies the acceleration to the velocity
                                objs[o].velx -= modif[0];
                                objs[o].vely -= modif[1];
                                // adds the velocity to the x and y axis
                                objs[o].x += objs[o].velx;
                                objs[o].y += objs[o].vely;
                                // calculating distance
                                let dx = objs[o].x - objs[xx].x;
                                let dy = objs[o].y - objs[xx].y;
                                let distance = (dx * dx + dy * dy).sqrt();
                                // makes sure 2 objs are not overlapping
                                if distance <= objs[o].size + objs[xx].size {
                                    // if they are they get sent back depending on the bounce
                                    objs[o].x -= objs[o].velx;
                                    objs[o].y -= objs[o].vely;
                                    objs[o].velx -= objs[o].velx*objs[o].bounce;
                                    objs[o].vely -= objs[o].vely*objs[o].bounce;
                                    //println!("cccccccccccccccccccccccccccccccccccclos");
                                    //thread::sleep(Duration::from_millis(1000));
                                    
                                   }
                                   // prints out some usefull information
                                println!("Acceleration: xvel: {} yvel: {}| Position: x: {} y: {} Rendering Queue Number(RQN): {} Name: {}",
                                modif[0],modif[1],objs[o].x,objs[o].y,o,objs[o].name);

                            }


                        }

                      //  println!("x{} y{} m{}",objs[0].x,objs[0].y,objs[0].mass);
                    //    println!("x!{} y!{} m!{}",objs[1].x,objs[1].y,objs[1].mass);


                    }
                    // clears the screen
                    if cl{
                        webview.eval("clear()").expect("error clear");
                    }
                    // renders all the objs
                    for x in 0..objs.clone().len(){
                        webview.eval(&format!("createCir({},{},{},'{}')",objs[x].x,objs[x].y,objs[x].size,objs[x].name)).expect("RENDERING ERROR");

                    }

                    //webview.eval(&format!("createCir({},{},{})",objs[1].x,objs[1].y,objs[1].size));
                    

                }
                _ => {
                    // here is my weird invoking for things that has allot of information to it
                  
                    
                    let newr = arg.split("||").collect::<Vec<&str>>(); // gets all the arguments
                    let mut skip = false;
                    match newr[0]{
                        "new" =>{
                            // makes sure the is no issue with the numbers
                        for x in newr.clone(){
                            if x == ""{
                                skip = true;
                            }
                        }
                        if !skip{
                            // creates a new planet
                            let cusplan: obj::obj = obj::obj{
                                name: newr[8].to_string(),
                                x: newr[1].parse::<f32>().expect("parse error") as f32,
                                y: newr[2].parse::<f32>().expect("parse error") as f32,
                                mass: newr[3].parse::<f32>().expect("parse error") as f32,
                                size: newr[4].parse::<f32>().expect("parse error") as f32,
                                velx: newr[5].parse::<f32>().expect("parse error") as f32,
                                vely: newr[6].parse::<f32>().expect("parse error") as f32,
                                bounce: newr[7].parse::<f32>().expect("parse error") as f32,
                            };
                            // prints out all the information
                            println!("cs: {}",cusplan.x);
                            println!("cs: {}",cusplan.y);
                            println!("cs: {}",cusplan.mass);
                            println!("cs: {}",cusplan.size);
                            println!("cs: {}",cusplan.velx);
                            println!("cs: {}",cusplan.vely);

                            println!("cs: {}",cusplan.bounce);

                            // pushes the new planet to the loop
                            objs.push(cusplan.clone());

                        }
                    }
                    // loading the solsim file
                    "loadsim" =>{
                        let newplan = simloader::loadsim(&format!("{}.solsim",newr[1]));
                        objs = newplan;
                    }
                    // change the gravity constant
                    "gc" =>{
                        grc = newr[1].parse::<f32>().expect("parse error") as f32;
                    }
                    // makes so that the js side can print
                    "log" =>{
                        println!("JSLOG: {}",newr[1]);
                    }
                    _ =>{
                        // if its not a thing it shows and error
                        errorwin::error();
                        println!("not a thing");
                    }
                }
                
                    
                   //println!("{}",objs[0].x);


                    //println!("ERRRRRRRRRRRRRRRRRDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD");
                },
            };
            Ok(())
        })
        .run()
        .unwrap();
     
}

fn gravity(fi: &obj::obj,fii:&obj::obj,gravconst: f32) -> Vec<f32>{
    // gravity alg
    let mut ax = 0.0;
    let mut ay = 0.0;

    let dy = fi.y - fii.y;
    let dx = fi.x - fii.y;
    //let dsq = dx * dx + dy * dy;
    //let dr = dsq.sqrt();
    let distance =  ( dx*dx + dy*dy ).sqrt();

   // if dsq > 3.0 {
        let force = gravconst * ((fi.mass * fii.mass) / distance.sqrt());
        ax += force * dx / distance / fi.mass;
        ay += force * dy / distance / fi.mass;
    //}

    let newcord = vec!(ax,ay);
    return newcord;
}
fn htmlloader() -> String{
    // this is a function that loads either the html or from the compiled aka from html.rs 
    // if however html.html exists in the src folder then it opens that instead
    if Path::new("./src/html.html").exists(){
        println!("From html file");
        return openfile::readFile("src/html.html");

    }else{
        println!("From compiled");
        return html::mm.to_string();
    }

}



