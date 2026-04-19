use std::io;
use std::collections::HashMap;

fn main() {
    /*
     * Using a hash map and vectors, create a text interface to allow a user to add 
     * employee names to a department in a company; for example, “Add Sally to Engineering” 
     * or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department 
     * or all people in the company by department, sorted alphabetically.
     */

    /* Create database and some departments */
    let mut database = HashMap::new();
    let eng_dep: HashMap<String, i32> = HashMap::new();
    let sales_dep: HashMap<String, i32> = HashMap::new();
    let mut employee_id: i32 = 0;
    database.insert(String::from("Engineering"), eng_dep);
    database.insert(String::from("Sales"), sales_dep);

    loop
    {
        /* Ask for an action to be performed by the user */
        println!("=== Company Listing Database ===");
        println!("Please type one of the following options to proceed.");
        println!("  I = Insert a new employee for a given department");
        println!("  L = List all employees in a given department");
        println!("  C = List all employees in the company");
        println!("  Q = Quit the program");

        let employee_name;
        let employee_dep;

        match read_line().as_str() {
            "I" => 
            {
                /* Ask for employee and department name */
                println!("Please type the employee's name:");
                employee_name = read_line();
                println!("Please type the employee's department:");
                employee_dep = read_line();

                /* Add employee to the database */
                if let Some(dep) = database.get_mut(&employee_dep) 
                {
                    if dep.get(&employee_name).is_none()
                    {
                        println!("  Employee Name: {}", employee_name);
                        println!("  Employee ID: {}", employee_id);
                        println!("  Department: {}", employee_dep);
                        dep.insert(employee_name, employee_id); 
                        println!("Addition sucessful");
                        employee_id += 1;
                    }
                    else
                    {
                        println!("Error - Employee already exists in database");
                    }
                }
                else 
                {
                    println!("Error - Non-existent department specified: {}", employee_dep);
                }
            },
            "L" =>
            {
                /* Ask for department name to list */
                println!("Please type a department's name");
                let employee_dep = read_line();

                /* Display all people in the given department */
                if let Some(dep) = database.get_mut(&employee_dep)
                {
                    println!("====== Employees in {} Department ======", employee_dep);
                    println!("Employee Name       Employee ID         ");
                    for (employee_name, employee_id) in dep
                    {
                        println!("{:>20}{:>20}", employee_name, employee_id);
                    }
                    println!("========================================");
                }
                else
                {
                    println!("Error - Non-existent department specified: {}", employee_dep);
                }

            },
            "C" =>
            {
                /* Display all people in the company */
                break;
            },
            "Q" => break,
            _   => todo!(),
        };
    }
    println!("Quitting program.");
}

fn read_line() -> String
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
