fn main() {
    let n = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Enter the number of rows to compute");
        std::process::exit(1);
    }).parse::<u64>()
    .unwrap_or_else(|_op| {
        println!("Bad input");
        std::process::exit(1);
    });
    for r in 0..=n {
        let mut row = vec![1];
        for i in 0..r {
            row.push(row[i as usize] * (r-i) / (i+1)); // n!/k!(n-k)! -> n!/(k+1)!(n-k-1)!
        }
        println!("{:?}", row); 
    }
}
