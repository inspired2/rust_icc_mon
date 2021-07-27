pub type CustomErr = Box<dyn std::error::Error + Send + Sync>;

pub fn from(msg: &str) -> CustomErr {
    CustomErr::from(std::io::Error::new(std::io::ErrorKind::Other, msg))
}
