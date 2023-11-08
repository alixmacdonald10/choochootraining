pub fn summarise_timings(timings: Vec<u128>) {
    
    let mut total: u128 = 0;
    for timing in &timings {
        total += timing;
    }
    let average = total / timings.len() as u128;
    println!("Total time taken: {:?} ns", total);
    println!("Average time taken: {:?} ns", average);
}

#[macro_export]
macro_rules! measure_run {
    (
        $self:expr, 
        $input:expr,
        $expected_result:expr
    ) => {

        {
            let start = std::time::Instant::now();

            let result = $self.run_specified($input);

            let duration = start.elapsed();
            println!("Time taken {:?} ns", duration.as_nanos());
        
            (result, duration.as_nanos())
        }
    };
    (
        $self:expr, 
        $input:expr, 
        $target:expr,
        $expected_result:expr
    ) => {

        {
            let start = std::time::Instant::now();

            let result = $self.run_specified($input, $target);

            let duration = start.elapsed();
            println!("Time taken {:?} ns", duration.as_nanos());
        
            (result, duration.as_nanos())
        }
    };
    
}
