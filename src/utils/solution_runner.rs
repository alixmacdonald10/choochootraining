// Solution Runner for Leetcode problems. Could be made into a single struct with Option for additional fields however this will change the signature of F and diverge from Leetcodes signature. So i have kept as seperate structs for now.

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

    pub fn run(&self) {
        for i in 0..self.inputs.len() {
            _ = self.run_specified(i);
        }
    }

    pub fn run_specified(&self, item: usize) -> U {
        let input = self.inputs[item].clone();
        let expected_result = self.expected_outputs[item].clone();

        let result = (self.solution_func)(input);
        assert_eq!(result, expected_result);
        result
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

    pub fn run(&self) {
        for i in 0..self.inputs.len() {
            _ = self.run_specified(i);
        }
    }

    pub fn run_specified(&self, item: usize) -> V {
        let input = self.inputs[item].clone();
        let target = self.targets[item].clone();
        let expected_result = self.expected_outputs[item].clone();

        let result = (self.solution_func)(input, target);
        assert_eq!(result, expected_result);
        result
    }
}