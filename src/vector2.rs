#[derive(Debug)] // Deriving the Debug trait for Employee
pub struct Employee {
    name: String,
    email_id: String,
    salary: u16,
}

impl Employee {
    // Define the 'new' method to create Employee instances
    pub fn abc(name: &str, email_id: &str, salary: u16) -> Self {
        Employee {
            name: name.to_string(),
            email_id: email_id.to_string(),
            salary,
        }
    }
}

pub fn struct_vector() {
    let mut employees: Vec<Employee> = Vec::new();

    let names = ["Alice", "James", "Bob", "Mike", "Oscar"];
    let emails = [
        "alice@sun.org",
        "james@sun.org",
        "bob@sun.org",
        "mike@sun.org",
        "oscar@sun.org",
    ];
    let salary = [10000, 20000, 30000, 40000, 50000];

    // Use the 'new' method to create Employee instances
    for i in 0..5 {
        employees.push(Employee::abc(names[i], emails[i], salary[i]));
    }

    // Print the employees
    println!("\nList of all Employees \n", );
    for employee in &employees {
        println!(" {:?}", employee);
    }

    //filter employees with salary ranging from 20000 to 40000 inclusive
    let matching_employees: Vec<_> = employees
        .iter()
        .filter(|e| e.salary >= 20000 && e.salary <=40000)
        .collect();
    
    //print employees withing the salary range
    println!("\nSalary range 20K-40K \n", );
    for employee in matching_employees {
        println!("Employees between salary 20K-40K {:?}", employee);
    }

    //updating salary of Bob
    println!("\n Updated salary of Bob \n", );
    for employee in &mut employees {
        if employee.email_id == "bob@sun.org".to_string() {
            employee.salary = 60000;
            println!("{:?}\n",employee );
        }

    }

    

}


