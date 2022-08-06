#[minutus::wrap(method = "distance", class_method = "new")]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    #[minutus::class_method]
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[minutus::method]
    pub fn distance(&self, other: &Point) -> f64 {
        (((self.x - other.x).abs().pow(2) + (self.y - other.y).abs().pow(2)) as f64).sqrt()
    }
}

#[no_mangle]
pub extern "C" fn mruby_example_gem_init(mrb: *mut minutus::mruby::minu_state) {
    __init_Point(mrb)
}
