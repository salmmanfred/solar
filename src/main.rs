use web_view::*;
use openfile;
mod html;
use std::time::Duration;
#[derive(Clone)] 
struct obj{
    name: String,
    x: f32,
    y: f32,
    mass: f32,
    size:f32,
    velx: f32,
    vely: f32,
    bounce: f32,
}


fn main(){
    let mut planet1: obj = obj{
        name: "planet1".to_string(),
        x: 500.0,
        y: 250.0,
        mass: 1.0,
        size: 10.0,
        velx: 2.1,
        vely: 0.0,
        bounce: 1.1,

    };

    let mut planet2: obj = obj{
        name: "planet2".to_string(),
        x: 500.0,
        y: 50.0,
        mass: 1.0,
        size: 10.0,
        velx: 1.5,
        vely: 0.0,
        bounce: 1.1,

    };
    
    let mut sun: obj = obj{
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

    let mut cl = true;


    let mut objs = vec!(planet1.clone(), sun.clone(),planet2.clone());

    web_view::builder()
        .title("Solar")
        .content(Content::Html(openfile::readFile("src/html.html")))
        .size(1500, 950)
        .resizable(true)
        .debug(false)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                
                "exit" => {
                    webview.exit();
                }
                "start" =>{
                    println!("Start");
                    webview.eval("start()").expect("start error");

                }
                "clear" =>{
                    //let fobjs: Vec<obj> = Vec::new();
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
                "run" =>{
                    //thread::sleep(Duration::from_millis(10));
                    //println!("xzczczxcxzczczxcxzcxz!{} y!{}",objs[0].x,objs[1].x);
                    
                    for o in 0..objs.len(){
                        
                        for xx in 0..objs.clone().len(){
                            //println!("FFFFFFFFFFFFFFFFFx{} y{}",xx,o);

                            if xx != o{
                                let modif = gravity(objs[o].clone(), objs[xx].clone());
                                objs[o].velx -= modif[0];
                                objs[o].vely -= modif[1];

                                objs[o].x += objs[o].velx;
                                objs[o].y += objs[o].vely;
                                
                                let dx = objs[o].x - objs[xx].x;
                                let dy = objs[o].y - objs[xx].y;
                                let distance = (dx * dx + dy * dy).sqrt();

                                if distance <= objs[o].size + objs[xx].size {
                                    objs[o].x -= objs[o].velx;
                                    objs[o].y -= objs[o].vely;
                                    objs[o].velx -= objs[o].velx*objs[o].bounce;
                                    objs[o].vely -= objs[o].vely*objs[o].bounce;
                                    //println!("cccccccccccccccccccccccccccccccccccclos");
                                    //thread::sleep(Duration::from_millis(1000));
                                    
                                   }

                                println!("FFFFFFFFFFFFFFFFFx!{} y!{} nn{}",modif[0],modif[1],o);

                            }

                        }

                      //  println!("x{} y{} m{}",objs[0].x,objs[0].y,objs[0].mass);
                    //    println!("x!{} y!{} m!{}",objs[1].x,objs[1].y,objs[1].mass);


                    }
                    if cl{
                        webview.eval("clear()").expect("error clear");
                    }
                    for x in 0..objs.clone().len(){
                        webview.eval(&format!("createCir({},{},{})",objs[x].x,objs[x].y,objs[x].size));

                    }

                    //webview.eval(&format!("createCir({},{},{})",objs[1].x,objs[1].y,objs[1].size));
                    

                }
                _ => {
                    //println!("ok");
                    
                    let newr = arg.split("||").collect::<Vec<&str>>();
                    let mut skip = false;
                    for x in newr.clone(){
                        if x == ""{
                            skip = true;
                        }
                    }
                    if !skip{
                        let cusplan: obj = obj{
                            name: "custom".to_string(),
                            x: newr[0].parse::<f32>().expect("parse error") as f32,
                            y: newr[1].parse::<f32>().expect("parse error") as f32,
                            mass: newr[2].parse::<f32>().expect("parse error") as f32,
                            size: newr[3].parse::<f32>().expect("parse error") as f32,
                            velx: newr[4].parse::<f32>().expect("parse error") as f32,
                            vely: newr[5].parse::<f32>().expect("parse error") as f32,
                            bounce: newr[6].parse::<f32>().expect("parse error") as f32,
                        };
                        println!("cs: {}",cusplan.x);
                        println!("cs: {}",cusplan.y);
                        println!("cs: {}",cusplan.mass);
                        println!("cs: {}",cusplan.size);
                        println!("cs: {}",cusplan.velx);
                        println!("cs: {}",cusplan.vely);

                        println!("cs: {}",cusplan.bounce);


                        objs.push(cusplan.clone());

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

fn gravity(fi:obj,fii:obj) -> Vec<f32>{
    let mut ax = 0.0;
    let mut ay = 0.0;

    let dy = fi.y - fii.y;
    let dx = fi.x - fii.y;
    let dsq = dx * dx + dy * dy;
    let dr = dsq.sqrt();

    //if dsq > 5.0 {
        let force = 0.02 * ((fi.mass * fii.mass) / dsq);
        ax += force * (dx / dr)/fi.mass;
        ay += force * (dy / dr)/fi.mass;
    //}

    let newcord = vec!(ax,ay);
    return newcord;
}



