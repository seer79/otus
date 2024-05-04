fn main() {
    let input = vec![1, 2, 3];
    let p = input.iter().map(|x| {
        print!("{}", x);
        x % 2
    });
    for i in p {
        print!("{}", i)
    }
}
