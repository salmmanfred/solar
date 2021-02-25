// this is the obj which is used by everything
#[derive(Clone)] 
pub struct obj{
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub mass: f64,
    pub size:f64,
    pub velx: f64,
    pub vely: f64,
    pub bounce: f64,
}
impl obj{
    pub fn gravity(&mut self, fii: &obj, gravconst:f64){
        let mut ax = 0.0;
        let mut ay = 0.0;

        let dy = self.y - fii.y;
        let dx = self.x - fii.y;
        let dsq = dx * dx + dy * dy;
        let dr = dsq.sqrt();
        let distance =  ( dx*dx + dy*dy ).sqrt();

    // if dsq > 3.0 {
    /* let mas1 = fi.mass * fi.size * fi.size / gravconst;
        let mas2 = fii.mass * fii.size * fii.size / gravconst;*/


            let force = gravconst * ((self.mass * fii.mass) / dsq);
            ax += force * ((dx / distance) / self.mass);
            ay += force * ((dy / distance) / self.mass);
            //println!("mass: {} {}",mas1,fi.name);
        //}
        self.velx -= ax;
        self.vely -= ay;
        /*self.x += self.velx;
        self.y += self.vely;*/
        //vec!(ax,ay)
    }
    pub fn updatePosition(&mut self){
        self.x += self.velx;
        self.y += self.vely;
    }
}