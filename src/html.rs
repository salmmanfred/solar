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
    }

    .outer>* {
        grid-column: 1 / 1;
        grid-row: 1 / 1;
    }

    .outer .below {
        z-index: 2;
        width: 400px;
        height: 200px;
        border: 1px solid gray;
        margin-top: 5px;

    }

    .outer .top {
        z-index: 1;
        width: 400px;
        height: 200px;
        border: 1px solid gray;
        margin-top: 5px;
    }
    .outer .below2 {
        z-index: 3;
        width: 400px;
        height: 200px;
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
        width: 400px;
        height: 200px;
        border: 1px solid gray;
    }

    .inbs {
        margin-top: 400px;
        position: absolute;
        float: right;
        width: 400px;
        height: 200px;
        border: 1px solid gray;
    }

    .stb {
        width: 100%;

    }
</style>

<body>
    <h1 id="talk"></h1>

    <canvas id="myCanvas" width="1000" height="900" style="border:1px solid #d3d3d3;">
        Your browser does not support the HTML canvas tag.</canvas>
    <div class="outer">
        <div class="top">
            Add custom planet: <br>
            X: <input type="number" id="x">
            Y: <input type="number" id="y"><br>
            Mass: <input type="number" id="mass"><br>
            size: <input type="number" id="size"><br>
            Velocity X: <input type="number" id="vx"><br>
            Velocity Y: <input type="number" id="vy"><br>
            Bounce: <input type="number" id="bounce"><br>



            <br>
            <input type="button" value="Push new" onclick="addnew()">

        </div>

        <div class="below">
            Settings:<br>

            <input type="button" value="clear" onclick="external.invoke('clear')"><br>
            <input type="button" value="Text on" id="tc" onclick="notext()"><br>
            <input id="cl" type="button" value="Dont clear" onclick="external.invoke('cl')">



        </div>
        <div class="below2">
            Load custom solar simulation: <br>
            <input type="text" value="file" id="filename">
            <br>

            <input type="button" value="Load Simulation" onclick="loadsim()">



        </div>
        <input type="button" value="Start" onclick="external.invoke('start')" class="start">

    </div>

    <!--<div class="inb" id="list">
        list:

        <input type="button" value="clear" onclick="external.invoke('clear')">


    </div>-->






    <script>
        var c = document.getElementById("myCanvas");
        var ctx = c.getContext("2d");
        var on = true
        list = [];
        rendtext = true;
        function start() {
            if (on) {
                setInterval(function core() {
                    external.invoke('run');
                }, 10);
                /*setInterval(function lis() {
                    external.invoke('list');
                }, 100);
                on = false;*/
            }

        }
        
        function createCir(x, y, sx,name) {
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
            ctx.clearRect(0, 0, 1000, 900);
        }
        function addnew() {
            var x = document.getElementById("x").value;
            var y = document.getElementById("y").value;
            var vx = document.getElementById("vx").value;
            var vy = document.getElementById("vy").value;
            var mass = document.getElementById("mass").value;
            var size = document.getElementById("size").value;
            var bounce = document.getElementById("bounce").value;

            external.invoke("new||"+x.toString() + "||" + y.toString() + "||" + mass.toString() + "||" + size.toString() + "||" + vx.toString() + "||" + vy.toString() + "||" + bounce.toString());


        }

        function addlist(name, x, y) {

            para = document.createElement("div");
            para.innerText = "" + name + " x: " + x + "y: " + y;
            para.id = name;
            document.getElementById("list").appendChild(para)


        }
        

        function clears(name) {
            var i;
            if (name == '') {
                //document.getElementById("list").removeChild(getElementById(name));
            } else {
                document.getElementById(name).remove()

            }


        }
        
        
        
        function upbutcl(bool) {
            if (bool != "true") {
                document.getElementById("cl").value = "Clear";

            } else {
                document.getElementById("cl").value = "Dont Clear";

            }
        }
        function loadsim (){
            var filename = document.getElementById("filename").value;
         
            
            external.invoke("loadsim||"+filename);
        }
        function notext(){
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