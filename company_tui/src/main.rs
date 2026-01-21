use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!();
        println!("========================================");
        println!("        COMPANY TUI       ");
        println!("========================================");
        println!();
        println!("  [1] Add a new department");
        println!("  [2] Add a new employee");
        println!("  [3] View all employees");
        println!("  [4] Exit");
        println!();
        println!("----------------------------------------");
        print!("  Enter your choice: ");
        io::Write::flush(&mut io::stdout()).unwrap();

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
                println!();
                println!("--- Add New Department ---");
                print!("  Department name: ");
                io::Write::flush(&mut io::stdout()).unwrap();

                let mut department = String::new();
                io::stdin()
                    .read_line(&mut department)
                    .expect("Failed to read line");
                let department: String = department.trim().to_string();
                departments.insert(department, Vec::new());
            }
            2 => {
                if departments.is_empty() {
                    println!();
                    println!("  ! No departments found. Add one first.");
                    continue;
                }

                let mut department_names: Vec<&str> = Vec::new();
                for (department, _) in &departments {
                    department_names.push(department.as_str());
                }

                println!();
                println!("--- Add New Employee ---");
                println!("  Select a department:");
                for (i, department) in department_names.iter().enumerate() {
                    println!("    [{i}] {department}");
                }
                println!();
                print!("  Department #: ");
                io::Write::flush(&mut io::stdout()).unwrap();

                let mut department_choice = String::new();
                io::stdin()
                    .read_line(&mut department_choice)
                    .expect("Failed to read line");
                let department_choice: u32 = match department_choice.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("  ! Invalid choice");
                        continue;
                    }
                };

                let department_name = match department_names.get(department_choice as usize) {
                    Some(&department) => department.to_string(),
                    None => {
                        println!("  ! Invalid choice");
                        continue;
                    }
                };

                match departments.get_mut(&department_name) {
                    Some(department) => {
                        print!("  Employee name: ");
                        io::Write::flush(&mut io::stdout()).unwrap();

                        let mut employee = String::new();
                        io::stdin()
                            .read_line(&mut employee)
                            .expect("Failed to read line");
                        let employee: String = employee.trim().to_string();
                        department.push(employee);
                    }
                    None => println!("  ! Invalid department"),
                }
            }
            3 => {
                println!();
                println!("========================================");
                println!("           COMPANY TUI            ");
                println!("========================================");

                if departments.is_empty() {
                    println!("  (empty)");
                } else {
                    for (department, employees) in &departments {
                        println!();
                        println!("  [{}]", department);
                        if employees.is_empty() {
                            println!("    - (no employees)");
                        } else {
                            for employee in employees {
                                println!("    - {}", employee);
                            }
                        }
                    }
                }
            }
            4 => {
                println!();
                println!("  Goodbye!");
                println!();
                break;
            }
            _ => println!("  ! Invalid choice"),
        }
    }
}
