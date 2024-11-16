use rand::Rng;


pub fn degress_to_radians(degress: f64) -> f64 {
    degress * std::f64::consts::PI / 180.0
}


pub fn random_double() -> f64
{
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}
