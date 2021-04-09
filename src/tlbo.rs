use crate::{Algorithm, AlgorithmBase, ObjFunc, Setting};

/// Teaching Learning Based Optimization settings.
/// This is a type alias to [Setting](struct.Setting.html).
pub type TLBOSetting = Setting;

/// Teaching Learning Based Optimization type.
pub struct TLBO<F: ObjFunc> {
    tmp: Vec<f64>,
    base: AlgorithmBase<F>,
}

impl<F: ObjFunc> TLBO<F> {
    pub fn new(func: F, settings: TLBOSetting) -> Self {
        let base = AlgorithmBase::new(func, settings);
        Self {
            tmp: zeros!(base.dim),
            base,
        }
    }
    fn register(&mut self, i: usize) {
        let f_new = self.base.func.fitness(self.base.gen, &self.tmp);
        if f_new < self.base.fitness[i] {
            self.base.pool[i] = self.tmp.clone();
            self.base.fitness[i] = f_new;
        }
        if f_new < self.base.best_f {
            self.set_best(i);
        }
    }
    fn teaching(&mut self, i: usize) {
        let tf = f64::round(rand!() + 1.);
        for s in 0..self.base.dim {
            let mut mean = 0.;
            for j in 0..self.base.pop_num {
                mean += self.base.pool[j][s];
            }
            mean /= self.base.dim as f64;
            self.tmp[s] = self.check(s, self.base.pool[i][s]
                + rand!(1., self.base.dim as f64) * (self.base.best[s] - tf * mean));
        }
        self.register(i);
    }
    fn learning(&mut self, i: usize) {
        let j = {
            let j = rand!(0, self.base.pop_num - 1);
            if j >= i { j + 1 } else { j }
        };
        for s in 0..self.base.dim {
            let diff = if self.base.fitness[j] < self.base.fitness[i] {
                self.base.pool[i][s] - self.base.pool[j][s]
            } else {
                self.base.pool[j][s] - self.base.pool[i][s]
            };
            self.tmp[s] = self.check(s, self.base.pool[i][s]
                + rand!(1., self.base.dim as f64) * diff);
        }
        self.register(i);
    }
}

impl<F: ObjFunc> Algorithm<F> for TLBO<F> {
    fn base(&self) -> &AlgorithmBase<F> { &self.base }
    fn base_mut(&mut self) -> &mut AlgorithmBase<F> { &mut self.base }
    fn generation(&mut self) {
        for i in 0..self.base.pop_num {
            self.teaching(i);
            self.learning(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        {Task, TLBO, TLBOSetting},
        tests::{test, TestObj},
    };

    #[test]
    fn tlbo() {
        test(TLBO::new(
            TestObj::new(),
            TLBOSetting {
                task: Task::MinFit(1e-20),
                ..Default::default()
            },
        ));
    }
}
