struct SAudio {
    audiosourcelocation: Vec<f64>,
    db: f64,
    distance: f64,
    speaker: Vec<Speaker>
}

impl SAudio {
    fn get_db_by_distance(&self, seconddistance: f64) -> f64 {
        self.db - (20.0 * (self.distance/seconddistance).log(10.0)).abs()
    }
    fn get_soundpressure(&self) -> f64 {
        2.0_f64/(10.0_f64).powf(5.0_f64)*(10.0_f64).powf(self.db/20.0_f64)
    }
    fn get_maximum_soundpressure_from_array(&self) -> f64 {
        let mut x: f64 = 0.0;
        for i in &self.speaker {
            x += 10.0_f64.powf(i.get_maximum_soundpressure()/10.0_f64)
        }
        10.0_f64*x.log(10.0_f64)
    }
}
struct Speaker {
    power: f64,
    sensitivity: f64 //Kennschalldruck
}
impl Speaker {
    fn new(power: f64, sensitivity: f64) -> Speaker {
        Speaker {power: power, sensitivity: sensitivity}
    }
    fn get_acoustic_efficiency(&self, acousticpower: f64) -> f64 {
        acousticpower/self.power
    }
    fn get_maximum_soundpressure(&self) -> f64 {
        10.0*self.power.log(10.0)+self.sensitivity
    }
}

fn main() {
    let mut a = Vec::from([
        Speaker::new(1000.0, 98.0),
        Speaker::new(1000.0, 98.0),
        Speaker::new(1000.0, 98.0)
    ]);
    let s = SAudio {
        audiosourcelocation: Vec::from([3.0, 1.0, 2.0]),
        db: 100.0,
        distance: 1.0,
        speaker: a
    };
    println!("Hello, world! {}", s.get_maximum_soundpressure_from_array());
}
