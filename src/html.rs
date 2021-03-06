// storing all the html files so they are compiled with the program
use std::path::Path;

#[derive(Clone)] 
pub struct html{
    pub html: String,
    pub im: String,
}
impl html{
    pub fn html(&self) -> String{
        if Path::new(&self.html).exists(){
            println!("From html file");
            return openfile::readFile(&self.html);
    
        }else{
            println!("From compiled");
            return self.im.clone();
        }
    }
}


pub const mm: &str = r#"
<!DOCTYPE html>
<html>
<style>
    body {
        font-family: 'Trebuchet MS', 'Lucida Sans Unicode', 'Lucida Grande', 'Lucida Sans', Arial, sans-serif;
  
    }



    .outer {
        display: grid;
        grid-template: 1fr / 1fr;
        place-items: center;
        float: right;
        width: 25%;
    }

    .outer>* {
        grid-column: 1 / 1;
        grid-row: 1 / 1;
    }

    .outer .below {
        z-index: 2;
        width: 100%;

        height: 200px;
        border: 1px solid gray;
        margin-top: 5px;

    }

    .outer .top {
        z-index: 1;
        width: 100%;

        height: 220px;
        border: 1px solid gray;
        margin-top: 5px;
    }
    .outer .below2 {
        z-index: 3;
       
        width: 100%;

        height: 70px;
        border: 1px solid gray;
        margin-top: 5px;

    }

    .outer .start {
        z-index: 4;
        width: 100%;
        margin-top: 5px;
        border: 2px solid black;

    }

    .inb {
        position: absolute;

        float: right;
               width: 100%;

        height: 200px;
        border: 1px solid gray;
    }

    .inbs {
        margin-top: 400px;
        position: absolute;
        float: right;
               width: 100%;

        height: 200px;
        border: 1px solid gray;
    }

    .stb {
        width: 100%;

    }
    .wid100{
        width: 100px;
      
    }
    .wid200{
        width: 23%;
        
    }
    
</style>

<body>
    <h1 id="talk"></h1>

    <canvas id="myCanvas" width="75%" height="100%" style="border:1px solid #d3d3d3; background-color: white;">
        Your browser does not support the HTML canvas tag.</canvas>
    <div class="outer">
        <div class="top">
            Add custom object: <br>
            Name: <input type="text" id="name"><br>
            X: <input type="number" id="x">
            Y: <input type="number" id="y"><br>
            Mass: <input type="number" id="mass"><br>
            size: <input type="number" id="size"><br>
            Velocity X: <input type="number" id="vx"><br>
            Velocity Y: <input type="number" id="vy"><br>
            Bounce: <input type="number" id="bounce"><br>

            
            <input type="button" value="Push new" onclick="addnew()">

        </div>

        <div class="below">
            Settings:<br>

            <input type="button" value="Clear Objects" onclick="external.invoke('clear')"><br>
            <input type="button" value="Text off" id="tc" onclick="notext()"><br>
            Update Freq: <input type="number" id="speed" value="10"><br>
            Gravity Constant: <input type="number" id="gc" value="0.00002">
            <input type="button" value="Submit"  onclick="grv()"><br>

            <input id="cl" type="button" value="Dont clear" onclick="external.invoke('cl')">



        </div>
        <div class="below" id="list">
            List of objects:
    
            <!--<input type="button" value="clear" onclick="external.invoke('clear')">-->
    
    
        </div>
        <div class="below2">
            Load custom solar simulation: <br>
            <input type="text" value="file" id="filename">
            <br>

            <input type="button" value="Load Simulation" onclick="loadsim()">



        </div>
        <input type="button" value="Start" onclick="external.invoke('start')" class="start">
        <input type="button" value="Pause" onclick="external.invoke('pause')" class="start">


    </div>

    






    <script>
        //This is the frontend
        // Here it gets the canvas and then get the 2d element of it 

        var c = document.getElementById("myCanvas");
        var ctx = c.getContext("2d");
        // variable to make sure only one loop is started
        var on = true
        list = [];
        // if the text on the planets should be renderd
        rendtext = true;
       /* xsize = window.innerWidth*0.72;
        ysize = window.innerWidth*0.95;*/


        window.addEventListener('resize', resizeCanvas, false);

        function resizeCanvas() {
                // makes sure the screen is the correct size and the clear area is correct
                xsize = window.innerWidth*0.72;
                ysize = window.innerHeight*0.95;
                c.width = xsize;
                c.height = ysize;
                
                
        }
        resizeCanvas();
        external.invoke('run');

        function start() {
            if (on) {
                // starts the loop which start the program  
                setInterval(function core() {
                    external.invoke('run');
                }, parseInt(document.getElementById("speed").value));
                /*setInterval(function lis() {
                    external.invoke('list');
                }, 100);*/
                on = false;
                setInterval(function core() {
                    external.invoke('list');
                }, 100);
            }

        }
        
        function createCir(x, y, sx,name) {
            // "render" function its function is to render a circle on to the screen
            ctx.beginPath();
            ctx.arc(x, y, sx, 0, 2 * Math.PI);
            ctx.stroke();
            if (rendtext){
                fs = sx/2;
                if (sx <= 40){
                    ctx.textAlign = "center";
                    ctx.font = sx*2+"px Arial";
                    ctx.fillText(name, x, y-sx*2); 
                }else{
                    ctx.textAlign = "center";
                    ctx.font = fs+"px Arial";
                    ctx.fillText(name, x, y); 
                }
            }
        }
        function clear() {
            //Clears the screen
            ctx.clearRect(0, 0, xsize, ysize);
        }
        function addnew() {
            var x = document.getElementById("x").value;
            var y = document.getElementById("y").value;
            var vx = document.getElementById("vx").value;
            var vy = document.getElementById("vy").value;
            var mass = document.getElementById("mass").value;
            var size = document.getElementById("size").value;
            var bounce = document.getElementById("bounce").value;
            var name = document.getElementById("name").value;

            // Gets all the values and invokes them to the rust side where they are processed into a new obj
            external.invoke("new||"+x.toString() + "||" + y.toString() + "||" + mass.toString() + "||" + size.toString() + "||" + vx.toString() + "||" + vy.toString() + "||" + bounce.toString()+"||"+name.toString());


        }
        function grv() {
            //gets the gravity when you submit it
            var x = document.getElementById("gc").value;
            

            external.invoke("gc||"+x);


        }

        function addlist(name, x, y) {

            para = document.createElement("div");
            parax = document.createElement("a");
            paray = document.createElement("a");

            para.innerText = "" + name +": "
            parax.innerText = "X:" + x+ " "
            paray.innerText = "Y:" + y + " "
           
         

            // + " X: " + x + " Y: " + y;
            para.id = "del";
            para.appendChild(parax).appendChild(paray)
            document.getElementById("list").appendChild(para)


        }
        

        function clears() {
           
                document.getElementById("list").removeChild(document.getElementById("del"))

            


        }
        
        
        
        function upbutcl(bool) {
            // makes sure the button id="cl" is showing the correct text
            if (bool != "true") {
                document.getElementById("cl").value = "Clear";

            } else {
                document.getElementById("cl").value = "Dont Clear";

            }
        }
        function loadsim (){
            //gets the file name of the file you want to load for a sim
            var filename = document.getElementById("filename").value;
         
            
            external.invoke("loadsim||"+filename);
            external.invoke('run');

        }
        function notext(){
            // same as upbutcl()
            rendtext = !rendtext;
            if (rendtext != true) {
                document.getElementById("tc").value = "Text on";

            } else {
                document.getElementById("tc").value = "Text off";

            }
        }

        

    </script>

</body>

</html>
"#;
pub const erh: &str = r#"
<html>
    <body>

        There has been an error
        <input class="but" type="button" value="Close" onclick="external.invoke('exit')">

    </body>
</html>
"#;