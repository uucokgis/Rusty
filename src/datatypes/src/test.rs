/*
Using a hash map and vectors, create a text interface to allow a user to add employee names to a
department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let
the user retrieve a list of all people in a department or all people in the company by department,
sorted alphabetically.
*/

use std::io;
use std::io::Read;


fn main () {
    enum DepartmentNames {
        ResearchAndDevelopment,
        Sales,
        ProjectManagement,
        SoftwareDevelopment
    }

    struct CompanyManagement {
        employee_list: Vec<String>,
        departments: DepartmentNames,

    }

    impl CompanyManagement {
        // we will fill it
        fn retrieve_employees (self) -> Vec<String>{
            return self.employee_list;
        }

        fn retrieve_department_names (self) -> DepartmentNames {
            return self.departments;
        }

        fn add_employee(self, employee_name: String) {

        }

        pub fn add_employee_console(&self) {
            let mut employee_name = String::new();
            io::stdin().read_line(& mut employee_name)
                .expect("Failed to reading your line");

            // pass

        }
    }

    // all together
    let employee_names: Vec<String> = Vec::new();

}