static mut MEMO: [u64; 101] = [0; 101];

fn main()
{
    let n: usize = 80;
    for i in 0..=n
    {
        print_fibo(i);
    }
}

fn print_fibo(n: usize)
{
    let fibo = fibonacci(n);
    println!("{n}th fibonacci number is {fibo}");
}

fn fibonacci(n: usize) -> u64
{
    if 1 == n || 0 == n
    {
        return n as u64;
    }
    unsafe //
    {
        if 0 == MEMO[n]
        {
            MEMO[n] = fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        MEMO[n]
    }
}
