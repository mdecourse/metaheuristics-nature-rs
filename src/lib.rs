//! A collection of nature-inspired metaheuristic algorithms.
//! ```
//! use metaheuristics_nature::{Report, RGA, RGASetting, Solver, Task, ObjFunc};
//! # use ndarray::{Array1, AsArray, ArrayView1};
//! # struct MyFunc(Array1<f64>, Array1<f64>);
//! # impl MyFunc {
//! #     fn new() -> Self { Self(Array1::zeros(3), Array1::ones(3) * 50.) }
//! # }
//! # impl ObjFunc for MyFunc {
//! #     type Result = f64;
//! #     fn fitness<'a, A>(&self, v: A, _: &Report) -> f64
//! #     where
//! #         A: AsArray<'a, f64>,
//! #     {
//! #         let v = v.into();
//! #         v[0] * v[0] + v[1] * v[1] + v[2] * v[2]
//! #     }
//! #     fn result<'a, V>(&self, v: V) -> Self::Result
//! #     where
//! #         V: AsArray<'a, f64>
//! #     {
//! #         self.fitness(v, &Default::default())
//! #     }
//! #     fn ub(&self) -> ArrayView1<f64> { self.1.view() }
//! #     fn lb(&self) -> ArrayView1<f64> { self.0.view() }
//! # }
//!
//! let a = RGA::solve(
//!     MyFunc::new(),
//!     RGASetting::default().task(Task::MinFit(1e-20)),
//!     () // Run without callback
//! );
//! let ans: f64 = a.result(); // Get the result from objective function
//! let (x, y): (Array1<f64>, f64) = a.parameters(); // Get the optimized XY value of your function
//! let history: Vec<Report> = a.history(); // Get the history reports
//! ```
//!
//! There are two traits [`Algorithm`] and [`Solver`].
//! The previous is used to design the optimization method,
//! and the latter is a simple interface for obtaining the solution, or analyzing the result.
//!
//! `Solver` will automatically implement for the type which implements `Algorithm`.
//!
//! # Objective Function
//!
//! You can define your question as a objective function through implementing [`ObjFunc`].
//!
//! First of all, the array types are [`ndarray::ArrayBase`].
//! And then you should define the upper bound, lower bound, and objective function [`ObjFunc::fitness`] by yourself.
//!
//! The final answer is [`ObjFunc::result`], which is generated from the design parameters.
//!
//! # Features
//!
//! + `parallel`: Enable parallel function, let objective function running without ordered,
//!   uses [`std::thread::spawn`].
//!   Disable it for the platform that doesn't supported threading,
//!   or if your objective function is not complicate enough.
pub use crate::callback::*;
pub use crate::methods::*;
pub use crate::obj_func::*;
pub use crate::utility::*;

/// Generate random values between [0., 1.) or by range.
#[macro_export]
macro_rules! rand {
    ($lb:expr, $ub:expr) => {{
        use rand::Rng;
        rand::thread_rng().gen_range($lb..$ub)
    }};
    () => {
        rand!(0., 1.)
    };
}

/// Generate random boolean by positive factor.
#[macro_export]
macro_rules! maybe {
    ($v:expr) => {{
        use rand::Rng;
        rand::thread_rng().gen_bool($v)
    }};
}

/// Define a data structure and its builder functions.
///
/// Use `@` to denote the base settings, such as population number, task category
/// or reporting interval.
/// ```
/// use metaheuristics_nature::setting_builder;
///
/// setting_builder! {
///     /// Real-coded Genetic Algorithm settings.
///     pub struct GASetting {
///         @base,
///         @pop_num = 500,
///         cross: f64 = 0.95,
///         mutate: f64 = 0.05,
///         win: f64 = 0.95,
///         delta: f64 = 5.,
///     }
/// }
/// let s = GASetting::default().pop_num(300).cross(0.9);
/// ```
#[macro_export]
macro_rules! setting_builder {
    (
        $(#[$attr:meta])*
        $v:vis struct $name:ident {
            $(@$base:ident, $(@$base_field:ident = $base_default:expr,)*)?
            $($(#[$field_attr:meta])* $field:ident: $field_type:ty = $field_default:expr,)+
        }
    ) => {
        $(#[$attr])*
        $v struct $name {
            $($base: $crate::Setting,)?
            $($field: $field_type,)+
        }
        impl $name {
            $(setting_builder! {
                @$base,
                /// Termination condition.
                task: $crate::Task,
                /// Population number.
                pop_num: usize,
                /// The report frequency. (per generation)
                rpt: u32,
            })?
            $($(#[$field_attr])* pub fn $field(mut self, $field: $field_type) -> Self {
                self.$field = $field;
                self
            })+
        }
        impl Default for $name {
            fn default() -> Self {
                Self {
                    $($base: $crate::Setting::default()$(.$base_field($base_default))*,)?
                    $($field: $field_default,)+
                }
            }
        }
    };
    (@$base:ident, $($(#[$field_attr:meta])* $field:ident: $field_type:ty,)+) => {
        $($(#[$field_attr])* pub fn $field(mut self, $field: $field_type) -> Self {
            self.$base = self.$base.$field($field);
            self
        })+
    }
}

mod callback;
mod methods;
mod obj_func;
#[cfg(test)]
mod tests;
#[cfg(feature = "parallel")]
pub mod thread_pool;
mod utility;
