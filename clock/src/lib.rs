use std::fmt;
use std::fmt::Debug;

pub struct Clock {
    hour: i32,
    minute: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour %24, self.minute)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.minute == other.minute
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour %24, self.minute)
    }
}

impl Clock {
    pub fn new(hour: i32, minute: i32) -> Self {

        println!("new::starting h: {}, m: {}", hour, minute);

        let mut m = minute % 60;
        let mut h = hour + (minute / 60);
        h %= 24;
        println!("new::mid: {}, m: {}", h, m);

        if m < 0 {
            m += 60;
            h -= 1;
        }

        if h < 0 {
            h += 24;            
        } 

        println!("new::ending h: {}, m: {}", h, m);
        let hour = h % 24; 
        let minute = m; 

        Clock {hour, minute}
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        println!("add_minutes::starting h: {}, m: {}", self.hour, self.minute);
        self.minute += minutes;
        self.hour += self.minute / 60;
        self.minute %= 60;
        self.hour %= 24;    

        if self.minute < 0 {
            self.minute += 60;
            self.hour -= 1;
        }
        if self.hour < 0 {
            self.hour += 24;            
        }
        self.minute %= 60;
        self.hour %= 24;      
        println!("add_minutes::ending h: {}, m: {}", self.hour, self.minute);      
        Clock {hour: self.hour, minute: self.minute}
    }
}
