use std::io;
use std::collections::HashMap;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("1. Add a new department");
        println!("2. Add a new employee");
        println!("3. View all employees");
        println!("4. Exit");
        println!("Enter your choice:");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the name of the department:");
                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");
                let department: String = department.trim().to_string();
                departments.insert(department, Vec::new());
            },
            2 => {
                if departments.is_empty() {
                    println!("No departments found, please add a department first");
                    continue;
                }
                
                let department_name = {
                    let mut department_names: Vec<&str> = Vec::new();
                    for (department, _) in &departments {
                        department_names.push(department.as_str());
                    }

                    println!("Choose a department from the following list (enter the number):");
                    for (i, department) in department_names.iter().enumerate() {
                        println!("{i}. {department}");
                    }
                    let mut department_choice = String::new();
                    io::stdin()
                        .read_line(&mut department_choice)
                        .expect("Failed to read line");
                    let department_choice: u32 = match department_choice.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Invalid department choice");
                            continue;
                        }
                    };
                    
                    match department_names.get(department_choice as usize) {
                        Some(department) => department.to_string(),
                        None => {
                            println!("Invalid department choice");
                            continue;
                        }
                    }
                };

                match departments.get_mut(&department_name) {
                    Some(department) => {
                        println!("Enter the name of the employee:");
                        let mut employee = String::new();
                        io::stdin()
                            .read_line(&mut employee)
                            .expect("Failed to read line");
                        let employee: String = employee.trim().to_string();
                        department.push(employee);
                    }
                    None => println!("Invalid department choice"),
                }
            },
            3 => {
                for (department, employees) in &departments {
                    println!("Department: {department}");
                    for employee in employees {
                        println!("Employee: {employee}");
                    }
                }
            },
            4 => break,
            _ => println!("Invalid choice, please try again"),
        }
    }
}