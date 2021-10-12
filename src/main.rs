use std::{
    collections::VecDeque,
    io::{stdin, stdout, Write},
};

use rand::{thread_rng, Rng};

fn pause() {
    let mut stdout = stdout();
    stdout.write_all(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_line(&mut "".to_string()).unwrap();
}

fn main() {
    let mut time = 0;

    let mut landing_queue: VecDeque<Airplane> = VecDeque::new();

    let mut takeoff_queue: VecDeque<Airplane> = VecDeque::new();

    let mut landing_id = 1;

    let mut takeoff_id = 2;

    let mut rng = thread_rng();

    loop {
        let new_landing = rng.gen_bool(0.4);
        let new_takeoff = rng.gen_bool(0.4);

        let new_landing_planes = rng.gen_range(1..3);
        let new_takeoff_planes = rng.gen_range(1..3);

        if new_landing {
            for _num in 0..new_landing_planes {
                let plane = Airplane::new(landing_id);

                landing_queue.push_back(plane);

                landing_id += 2;
            }
        }

        if new_takeoff {
            for _num in 0..new_takeoff_planes {
                let plane = Airplane::new(takeoff_id);

                takeoff_queue.push_back(plane);

                takeoff_id += 2;
            }
        }

        println!("The time is {}:00.", time);

        println!("There are {} planes waiting to land.", landing_queue.len());
        println!(
            "There are {} planes waiting to take off.",
            takeoff_queue.len()
        );

        if !landing_queue.is_empty() {
            let landing_plane = landing_queue.pop_front().unwrap();

            println!("Plane #{} is cleared to land.", landing_plane.id);
        } else if !takeoff_queue.is_empty() {
            let taking_off_plane = takeoff_queue.pop_front().unwrap();

            println!("Plane #{} is cleared to takeoff.", taking_off_plane.id);
        } else {
            println!("No planes waiting, no planes land or take off.")
        }

        time += 1;
        pause()
    }
}

struct Airplane {
    id: i32,
}

impl Airplane {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}
