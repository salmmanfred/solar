use openfile;
use crate::obj;
use crate::errorwin;
// this the the function that loads the simulations from file
pub fn loadsim(file:&str) -> Vec<obj::obj>{
    let mut returndata: Vec<obj::obj> = Vec::new();
    let filecontent = openfile::readFileLines(file);
    
    for x in filecontent{
        // parse all the lines
        let data = x.split("||").collect::<Vec<&str>>();
        if data.len() == 8{
            // ^ makes sure its all 8 values
            // then creates the function
            let loadsimplan: obj::obj = obj::obj{
                name: data[0].to_string(),
                x:  data[1].parse::<f64>().expect("parse error") as f64,
                y: data[2].parse::<f64>().expect("parse error2") as f64,
                mass: data[3].parse::<f64>().expect("parse error3") as f64,
                size: data[4].parse::<f64>().expect("parse error4") as f64,
                velx: data[5].parse::<f64>().expect("parse error5") as f64,
                vely: data[6].parse::<f64>().expect("parse error6") as f64,
                bounce: data[7].parse::<f64>().expect("parse error7") as f64,
            

            };
            returndata.push(loadsimplan);
        }else{
            //errorwin::error();
            println!("PARSE ERROR");
            break;
        }

    }
    // returns the new vector
    
    return returndata;
}