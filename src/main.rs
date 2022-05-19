#[allow(unused_variables)]
#[allow(unused_assignments)]




static mut R: i32 = 0;
fn main() {

    {
        let a = 3;
        println!("{}",a);
    }
    unsafe { // A way to say Trust me, I know what I am doing to the Rust compiler
        R = 4;
        println!("R = {}", R);
    }
    unsafe {
        println!("R = {}", R);
    }
    //closures
    let a = |a:i32| a+1;
    println!("{}", a(6));
    let  b = |b:i32| -> i32 {
        let c = b+1;
        c
    };
    println!("{}", b(4));

    let gen = |x| println!("{}", x);
    //gen(3);
    gen(true);

    let square = |a:i32| a*a;
    apply(square, 6);

    //Calculate the sum of all sqaures less than 500
    //only for even numbers
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i*i;
        if isq > limit{break;}
        else {
            if is_even(isq){
                sum+=isq;
            }

        }
    }
    println!("The sum is {}", sum);


    // With HOFS
    let sum2 =
        (0..).map(|x| x*x).take_while(|&x| x <= limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum,x| sum + x);
    println!("The sum using HOFs is {}", sum2);




}

fn is_even(x:i32) -> bool{
    x % 2 == 0
}

//High Order Functions
fn apply(f:fn(i32) -> i32, a: i32){
    println!("Result {}", f(a));
}


