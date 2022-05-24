struct Socket {
    desc: String,
    pwr_on: bool,
    pwr: f32,
}

struct Thermometer {
    tmpr: f32,
}

trait Sockets {
    fn print_desc(&self) {}
    fn print_pwr(&self) {}
    fn turn_on(&mut self) {}
    fn turn_off(&mut self) {}
}

trait Thermometers {
    fn get_tmpr(&mut self) {}
    fn print_tmpr(&self) {}
}

impl Sockets for Socket {
    fn print_desc(&self) {
        println!("I'm {}", self.desc)
    }
    fn print_pwr(&self) {
        println!("{} / Power: {}", self.desc, self.pwr);
    }
    fn turn_on(&mut self) {
        self.pwr_on = true
    }
    fn turn_off(&mut self) {
        self.pwr_on = false
    }
}

impl Thermometers for Thermometer {
    fn get_tmpr(&mut self) {
        self.tmpr = 21.2121
    }
    fn print_tmpr(&self) {
        println!("Temperature: {:.2}", self.tmpr)
    }
}

fn main() {
    let socket1 = Socket {
        desc: String::from("1st socket"),
        pwr_on: false,
        pwr: 0.0, // read value from device
    };
    let therm1 = Thermometer {
        tmpr: 21.21, // read value from device
    };
    socket1.print_desc();
    socket1.print_pwr();
    therm1.print_tmpr()
}
