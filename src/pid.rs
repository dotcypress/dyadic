use crate::DF;

pub struct Regulator {
    kp: DF,
    ki: DF,
    kd: DF,
    last_error: DF,
    error_sum: DF,
    min_error_sum: DF,
    max_error_sum: DF,
}

impl Regulator {
    pub fn new(
        kp: impl Into<DF>,
        ki: impl Into<DF>,
        kd: impl Into<DF>,
        min_error_sum: impl Into<DF>,
        max_error_sum: impl Into<DF>,
    ) -> Self {
        Self {
            kp: kp.into(),
            ki: ki.into(),
            kd: kd.into(),
            min_error_sum: min_error_sum.into(),
            max_error_sum: max_error_sum.into(),
            last_error: DF::default(),
            error_sum: DF::default(),
        }
    }

    pub fn update(&mut self, sp: impl Into<DF>, val: impl Into<DF>) -> DF {
        let error = sp.into() - val.into();
        let error_delta = error - self.last_error;

        self.last_error = error;
        self.error_sum = DF::clamp(
            self.error_sum + error,
            self.min_error_sum,
            self.max_error_sum,
        );

        let p = error * self.kp;
        let i = self.error_sum * self.ki;
        let d = error_delta * self.kd;

        p + i + d
    }
}
