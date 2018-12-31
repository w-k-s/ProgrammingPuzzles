use std::fs;

#[derive(Debug)]
struct Employee<'a> {
    name: &'a str,
    subordinates: Vec<Box<Employee<'a>>>,
}

impl<'a> Employee<'a> {
    fn new(name: &'a str) -> Employee<'a> {
        return Employee {
            name: name,
            subordinates: vec![],
        };
    }

    fn find_subordinate(&mut self, name: &str) -> Option<&mut Employee<'a>> {

        if self.name == name {
            return Some(self);
        }
        if self.subordinates.is_empty() {
            return None;
        }
        for employee in self.subordinates.iter_mut() {
            if let Some(employee) = (**employee).find_subordinate(name) {
                return Some(employee);
            }
        }
        return None;
    }

    fn find_first_manager_of_(&self,names: Vec<&str>)->Option<&Employee<'a>>{    
        //check if self has subordinates
        if self.subordinates.is_empty(){   
            return None;
        }

        //check if self is direct manager of one of given names
        let matching_names_count = self.subordinates.iter().filter(|e| names.contains(&e.name)).count();
        if matching_names_count > 0{
            return Some(self);
        }
        
        //check if self if the first_manager of subordinates
        for employee in self.subordinates.iter(){
            print!("Considering {:?}\n",employee.name);
            if let Some(manager) = employee.find_first_manager_of_(names.clone()){
                print!("-- Considering {:?}\n",manager);
                //return manager if is direct manager of one of the given names
                if manager.subordinates.iter().filter(|e| names.contains(&e.name)).count() > 0 {
                    return Some(&manager);
                }
            }
        }
        None
    }
}

fn main() -> Result<(), String> {
    let contents = fs::read_to_string("input.txt").map_err(|err| format!("{}", err))?;

    let lines: Vec<&str> = contents.split('\n').collect();

    if lines.is_empty() {
        return Ok(());
    }

    let names = &lines[1..];

    //Set root
    let top_relationship: Vec<&str> = names[2].split(' ').collect();

    let mut head = Employee::new(top_relationship[0]);
    let employee = Employee::new(top_relationship[1]);
    head.subordinates.push(Box::new(employee));

    //Build Hierarchy
    for relationship in &names[3..] {
        let members: Vec<&str> = relationship.split(' ').collect();
        let manager = members[0];
        let subordinate = members[1];

        if let Some(manager) = head.find_subordinate(manager) {
            (*manager).subordinates.push(Box::new(Employee::new(subordinate)))
        }
    }
    
    //Find first manager
    match head.find_first_manager_of_(vec![names[0],names[1]]){
        Some(manager)=>print!("{}\n",manager.name),
        None=>print!("None\n")
    };

    Ok(())
}
