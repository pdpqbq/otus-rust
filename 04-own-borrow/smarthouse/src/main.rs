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
    fn turn_on(&mut self) {}
    fn turn_off(&mut self) {}
    fn get_pwr(&mut self) {}
}

trait Thermometers {
    fn get_tmpr(&mut self) {}
    fn print_tmpr(&self) {}
}

impl Sockets for Socket {
    fn print_desc(&self) {
        println!("{}", self.desc)
    }
    fn turn_on(&mut self) {
        self.pwr_on = true
    }
    fn turn_off(&mut self) {
        self.pwr_on = false
    }
    fn get_pwr(&mut self) {
        self.pwr = 1.1; // read value from device
        println!("{}", self.pwr)
    }
}

impl Thermometers for Thermometer {
    fn get_tmpr(&mut self) {
        self.tmpr = 21.2121
    }
    fn print_tmpr(&self) {
        println!("{:.2}", self.tmpr)
    }
}

fn main() {
    let mut socket1 = Socket {
        desc: String::from("1st socket"),
        pwr_on: false,
        pwr: 0.0, // read value from device
    };
    let therm1 = Thermometer {
        tmpr: 21.21, // read value from device
    };
    socket1.print_desc();
    socket1.get_pwr();
    therm1.print_tmpr()
}
