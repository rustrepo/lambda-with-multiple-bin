pub fn demonstrate_vector_operations() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);

    let x = v.get(1);
    match x {
        Some(x) => println!("The second element is {}", &x),
        None => println!("There is no second element"),
    }

    println!("Removing the second element from the vector using pop.");
    v.pop();

    let x = v.get(1);
    match x {
        Some(x) => println!("{}", &x),
        None => println!("Now, there is no second element"),
    }
}

pub fn iterating_vector() {

    let v = vec![1,2,3,4,5];

    for numbers in &v {
        println!("{}", numbers );
    }

    let even_numbers: Vec<_> = v.iter()
                                .filter(|&&x| x%2==0)
                                .collect();

    println!("{:?}", even_numbers );

       
    }
    


