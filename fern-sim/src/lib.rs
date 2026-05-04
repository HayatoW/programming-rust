pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    /// シダ植物の1日での成長をシミュレート
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

/// シダ植物シミュレーションを何日分か行う
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
