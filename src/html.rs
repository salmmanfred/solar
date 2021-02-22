pub const mm: &str = r#"
<!DOCTYPE html>
<html>

<body>

    <canvas id="myCanvas" width="500" height="500" style="border:1px solid #d3d3d3;">
        Your browser does not support the HTML canvas tag.</canvas>

    <input type="button" value="Start" onclick="external.invoke('start')">



    <script>
        var c = document.getElementById("myCanvas");
        var ctx = c.getContext("2d");
        function createCir(x, y, sx) {
            ctx.beginPath();
            ctx.arc(x, y, sx, 0, 2 * Math.PI);
            ctx.stroke();
        }
        function clear() {
            ctx.clearRect(0, 0, 500, 500);
        }

    </script>

</body>

</html>


"#;