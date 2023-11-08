// Solution Runner for Leetcode problems. Could be made into a single struct with Option for additional fields however this will change the signature of F and diverge from Leetcodes signature. So i have kept as seperate structs for now.

use crate::utils::timings;
use crate::measure_run;

pub struct Solution<T, U, F>
where
    T: Clone,
    U: Clone + std::fmt::Debug + PartialEq,
    F: Fn(T) -> U,
{
    inputs: Vec<T>,
    expected_outputs: Vec<U>,
    solution_func: F,
}


impl<T, U, F> Solution<T, U, F>
where
    T: Clone,
    U: Clone + std::fmt::Debug + PartialEq,
    F: Fn(T) -> U,
{
    pub fn new(inputs: Vec<T>, expected_outputs: Vec<U>, solution_func: F) -> Self {
        Self {
            inputs,
            expected_outputs,
            solution_func,
        }
    }

    pub fn run(&self, with_timings: bool) {

        let mut timings: Vec<u128> = Vec::with_capacity(self.inputs.len());
        for i in 0..self.inputs.len() {
            let input = self.inputs[i].clone();
            let expected_result = self.expected_outputs[i].clone();

            if with_timings {
                println!("Run {:?}: ", i);
                let (result, timing) = measure_run!(self, input.clone(), expected_result);
                timings.push(timing);
                assert_eq!(result, expected_result);
            } else {
                let result = self.run_specified(input);
                assert_eq!(result, expected_result);
            }    
        }

        if with_timings {
            timings::summarise_timings(timings);
        }
    }

    pub fn run_specified(&self, input: T) -> U {
        (self.solution_func)(input)
    }
}


pub struct SolutionWithTarget<T, U, V, F>
where
    T: Clone,
    U: Clone,
    V: Clone + std::fmt::Debug + PartialEq,
    F: Fn(T, U) -> V,
{
    inputs: Vec<T>,
    targets: Vec<U>,
    expected_outputs: Vec<V>,
    solution_func: F,
}


impl<T, U, V, F> SolutionWithTarget<T, U, V, F>
where
    T: Clone,
    U: Clone,
    V: Clone + std::fmt::Debug + PartialEq,
    F: Fn(T, U) -> V,
{
    pub fn new(inputs: Vec<T>, targets: Vec<U>, expected_outputs: Vec<V>, solution_func: F) -> Self {
        Self {
            inputs,
            targets,
            expected_outputs,
            solution_func,
        }
    }

    pub fn run(&self, with_timings: bool) {

        let mut timings = Vec::with_capacity(self.inputs.len());
        for i in 0..self.inputs.len() {
            let input = self.inputs[i].clone();
            let target = self.targets[i].clone();
            let expected_result = self.expected_outputs[i].clone();


            if with_timings {
                println!("Run {:?}: ", i);
                let (result, timing) = measure_run!(self, input.clone(), target, expected_result);
                timings.push(timing);
                assert_eq!(result, expected_result);
            } else {
                let result = self.run_specified(input, target);
                assert_eq!(result, expected_result);
            }
        }


        if with_timings {
            timings::summarise_timings(timings);
        }

    }

    pub fn run_specified(&self, input: T, target: U) -> V {
        (self.solution_func)(input, target)
    }
}