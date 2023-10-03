// extern crate regex;
//
// use self::regex::Regex;
use std::str::FromStr;
use std::str::SplitWhitespace;

#[derive(Debug, Clone, PartialEq)]
enum Service {
    Engineering,
    Marketing,
    Sales,
}

#[derive(Debug, Clone)]
pub struct Employee {
    name: String,
    service: Service,
}

// https://www.reddit.com/r/rust/comments/2vqama/parse_string_as_enum_value/cojzafn/
impl FromStr for Service {
    type Err = ();

    fn from_str(s: &str) -> Result<Service, ()> {
        match s {
            "engineering" => Ok(Service::Engineering),
            "marketing" => Ok(Service::Marketing),
            "sales" => Ok(Service::Sales),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut employees: Vec<Employee> = vec![];

    // Test fail cases
    parse_command(&mut employees, "");
    parse_command(&mut employees, "Toto");
    parse_command(&mut employees, "Add Toto");
    parse_command(&mut employees, "Add Toto Sales");
    parse_command(&mut employees, "Add Toto to Streamer");


    parse_command(&mut employees, "Add Sally toto Engineering");
    parse_command(&mut employees, "Add Amir to Sales");
    parse_command(&mut employees, "Add Sarah to Marketing");
    parse_command(&mut employees, "Add Amir to Engineering");
    parse_command(&mut employees, "Add Paul to Engineering");
    parse_command(&mut employees, "Add Paul to Engineering");
    println!("{:?}", employees);

    parse_command(&mut employees, "Remove Amir from Sales");
    parse_command(&mut employees, "Remove Paul from Engineering");
    parse_command(&mut employees, "Remove Sarah from Sales");
    println!("{:?}", employees);

    println!("{:?}", parse_command(&mut employees, "List Engineering employees"));
    println!("{:?}", parse_command(&mut employees, "List Marketing employees"));
    println!("{:?}", parse_command(&mut employees, "List Sales employees"));

    println!("{:?}", parse_command(&mut employees, "List all employees"));
}


pub fn parse_command(employees: &mut Vec<Employee>, cmd: &str) -> Vec<Employee> {
    let cmd = String::from(cmd);
    let mut iter = cmd.split_whitespace();

    let employees = match iter.next() {
        Some(a) => match a.to_lowercase().as_str() {
            "add" => add_employee(employees, iter),
            "remove" => remove_employee(employees, iter),
            "list" => list_employees(employees.to_vec(), iter),
            _ => {
                println!("Valid actions are Add, Remove and List.");
                employees.to_vec()
            }
        },
        None => {
            println!("You need to write an action");
            employees.to_vec()
        }
    };

    employees
}

fn add_employee(employees: &mut Vec<Employee>, mut iter: SplitWhitespace) -> Vec<Employee> {
    let mut new_employee = Employee {
        name: String::from(""),
        service: Service::Engineering,
    };
    new_employee.name = if let Some(name) = iter.next() {
        String::from(name)
    } else {
        println!("Enter the employee's name and service.");
        return employees.to_vec();
    };

    if let None = iter.next() {
        println!(r#"You need the "to" preposition in order to form an intelligible sentence."#);
        return employees.to_vec();
    };

    new_employee.service = if let Some(service) = iter.next() {
        match Service::from_str(&service.to_lowercase()) {
            Ok(srv) => srv,
            Err(_) => {
                println!("Precise the employee's service: Engineering, Marketing, or Sales.");
                return employees.to_vec();
            }
        }
    } else {
        println!("Precise the employee's service: Engineering, Marketing, or Sales.");
        return employees.to_vec();
    };

    println!("Adding {} to {:?}", new_employee.name, new_employee.service);
    employees.push(new_employee);
    employees.to_vec()
}

fn remove_employee(employees: &mut Vec<Employee>, mut iter: SplitWhitespace) -> Vec<Employee> {
    let mut delete_employee = Employee {
        name: String::from(""),
        service: Service::Engineering,
    };
    delete_employee.name = if let Some(name) = iter.next() {
        String::from(name)
    } else {
        println!("Enter the employee's name and service.");
        return employees.to_vec();
    };

    if let None = iter.next() {
        println!(r#"You need the "from" preposition in order to form an intelligible sentence."#);
        return employees.to_vec();
    };

    delete_employee.service = if let Some(service) = iter.next() {
        match Service::from_str(&service.to_lowercase()) {
            Ok(srv) => srv,
            Err(_) => {
                println!("Precise the employee's service: Engineering, Marketing, or Sales.");
                return employees.to_vec();
            }
        }
    } else {
        println!("Precise the employee's service: Engineering, Marketing, or Sales.");
        return employees.to_vec();
    };

    let deleted = employees
        .iter_mut()
        .position(|n| n.name == delete_employee.name && n.service == delete_employee.service);
    match deleted {
        Some(x) => {
            let removed = employees.remove(x);
            println!("Fired {:?} !", removed);
        }
        None => println!("No employee fired."),
    };

    employees.to_vec()
}

fn list_employees(employees: Vec<Employee>, mut iter: SplitWhitespace) -> Vec<Employee> {
    let service = if let Some(service) = iter.next() {
        match Service::from_str(&service.to_lowercase()) {
            Ok(srv) => Some(srv),
            Err(_) => {
                if &service.to_lowercase() == "all" {
                    None
                } else {
                    println!(
                        "Precise the employee's service: Engineering, Marketing, Sales or all."
                    );
                    // return employees;
                    None
                }
            }
        }
    } else {
        println!("Precise the employee's service: Engineering, Marketing, Sales or all.");
        // return Err("Precise the employee's service: Engineering, Marketing, or Sales.");
        None
    };

    let mut list = employees.to_owned();
    if let Some(srv) = service {
        println!("Keeping only the employees from the selected service ({:?})", srv);
        list.retain(|empl| empl.service == srv);
    }

    list
}