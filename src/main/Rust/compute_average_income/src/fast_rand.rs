use std::iter::repeat_with;
use std::time::{Instant};
use fastrand::Rng;



#[derive(Debug)]
struct Address {
    street: String,
    postal_code: String,
    city: String,
    country: String,
}

#[derive(Debug)]
struct Employee {
    first_name: String,
    last_name: String,
    address: Address,
    salary: u64,
}

fn create_random_string_of_80_chars(rng: &mut Rng, char_pool: &[char]) -> String {
    repeat_with(|| char_pool[rng.usize(0..char_pool.len())]).take(80).collect()
}

fn create_random_employee(rng: &mut Rng, char_pool: &[char]) -> Employee {
    Employee {
        first_name: create_random_string_of_80_chars(rng, char_pool),
        last_name: create_random_string_of_80_chars(rng, char_pool),
        address: Address
        {
            street: create_random_string_of_80_chars(rng, char_pool),
            postal_code: create_random_string_of_80_chars(rng, char_pool),
            city: create_random_string_of_80_chars(rng, char_pool),
            country: create_random_string_of_80_chars(rng, char_pool),
        },
        salary: 1000,
    }
}

fn lookup_all_employees<'a>(rng: &'a mut Rng, number_of_all_employees: usize, char_pool: &'a [char])
                            -> impl Iterator<Item=Employee> + 'a {
        repeat_with(move || create_random_employee(rng, char_pool)).take(number_of_all_employees)
}

fn compute_average_income_of_all_employees(employees: impl Iterator<Item=Employee>)
                                           -> f64 {
    let (num_of_employees, sum_of_salaries) =
        employees.fold((0, 0),
                       |(counter, sum), employee| (counter + 1, sum + employee.salary)
                       );
    return (sum_of_salaries as f64) / (num_of_employees as f64);
}

pub fn benchmark() {
    println!("Benchmarking fast random number generator");
    let char_pool: Vec<_> =
        ('a'..'z')
            .chain('A'..'Z')
            .chain('0'..'9')
            .collect();

    let mut rng = fastrand::Rng::new();

    let nrs_of_employees = [1000, 10000, 100000, 1000000];
    for &nr_of_employees in &nrs_of_employees {
        let start_time = Instant::now();
        let average = compute_average_income_of_all_employees(lookup_all_employees(&mut rng,
            nr_of_employees, &char_pool[..],
        ));
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);
        println!("n={} Average = {} Duration = {}ms", nr_of_employees, average, duration.as_millis());
    }
    println!();
}
