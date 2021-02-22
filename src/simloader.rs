use openfile;
use crate::obj;
pub fn loadsim(file:&str) -> Vec<obj::obj>{
    let mut returndata: Vec<obj::obj> = Vec::new();
    let filecontent = openfile::readFileLines(file);
    
    for x in filecontent{
        let data = x.split("||").collect::<Vec<&str>>();
        if data.len() == 8{
            let loadsimplan: obj::obj = obj::obj{
                name: data[0].to_string(),
                x: data[1].parse::<f32>().expect("parse error1") as f32,
                y: data[2].parse::<f32>().expect("parse error2") as f32,
                mass: data[3].parse::<f32>().expect("parse error3") as f32,
                size: data[4].parse::<f32>().expect("parse error4") as f32,
                velx: data[5].parse::<f32>().expect("parse error5") as f32,
                vely: data[6].parse::<f32>().expect("parse error6") as f32,
                bounce: data[7].parse::<f32>().expect("parse error7") as f32,
            };
            returndata.push(loadsimplan);
        }else{
            println!("PARSE ERROR");
            break;
        }

    }
    
    return returndata;
}