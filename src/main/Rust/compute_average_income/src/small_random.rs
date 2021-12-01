use rand::{SeedableRng, thread_rng, Rng};
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use std::time::{Instant};



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

fn create_random_string_of_80_chars<R: Rng>(rng: &mut R, char_pool: &[char]) -> String {
    char_pool.choose_multiple(rng, 80).collect()
}

fn create_random_employee<R: Rng>(rng: &mut R, char_pool: &[char]) -> Employee {
    return Employee {
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
    };
}

fn lookup_all_employees<'a>(number_of_all_employees: u64, char_pool: &'a [char])
                            -> impl Iterator<Item=Employee> + 'a {
    let mut thread_rng = thread_rng();
    let mut rng = SmallRng::from_rng(&mut thread_rng).unwrap();

    return
        (0..number_of_all_employees)
            .map(move |_| { create_random_employee(&mut rng, char_pool) })
            .into_iter();
}

fn compute_average_income_of_all_employees(employees: impl Iterator<Item=Employee>)
                                           -> f64 {
    let (num_of_employees, sum_of_salaries) =
        employees.fold((0u64, 0u64),
                       |(counter, sum), employee| {
                           return (counter + 1, sum + employee.salary);
                       });
    return (sum_of_salaries as f64) / (num_of_employees as f64);
}

pub fn benchmark() {
    println!("Benchmarking small random number generator");
    let char_pool: Vec<_> =
        ('a'..'z')
            .chain('A'..'Z')
            .chain('0'..'9')
            .collect();


    let nrs_of_employees = [1000u64, 10000, 100000, 1000000];
    for nr_of_employees in &nrs_of_employees {
        let start_time = Instant::now();
        let average = compute_average_income_of_all_employees(lookup_all_employees(
            *nr_of_employees, &char_pool[..],
        ));
        let end_time = Instant::now();
        let duration = end_time.duration_since(start_time);
        println!("n={} Average = {} Duration = {}ms", nr_of_employees, average, duration.as_millis());
    }
    println!();
}
