extern crate std_semaphore;
extern crate rand;

use std_semaphore::Semaphore;
use std::thread;
use std::time::Duration;
use rand::{thread_rng, Rng};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

const MAX_CONSECUTIVE_TIMES: usize = 2;

struct Philosopher {
    name: String,
    id: usize
}
impl Philosopher {
    fn new(name: &str, id: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            id: id
        }
    }

    fn left_fork(&self) -> usize
    {
        self.id
    }

    fn right_fork(&self) -> usize
    {
        (self.id + 1) % 5
    }

    fn left_philosopher(&self) -> usize
    {
        (self.id + 4) % 5
    }

    fn right_philosopher(&self) -> usize
    {
        (self.id + 1) % 5
    }

    fn eat(&self, fork:Arc<Vec<Semaphore>>, eating_states:Arc<RwLock<Vec<bool>>>,
           times_philosopher_ate:Arc<RwLock<Vec<usize>>>) {
        loop {
            if let Ok(mut states_mut) = eating_states.write()
            {
                if let Ok(mut times_ate_mut) = times_philosopher_ate.write()
                {
                    println!("{:?}", *states_mut);
                    println!("Filosofo {} tratando de agarrar palito", self.left_fork());
                    println!("Times ate: {:?}", *times_ate_mut);

                    if (states_mut[self.id] == false
                        && states_mut[self.left_philosopher()] == false
                        && states_mut[self.right_philosopher()] == false
                        && times_ate_mut[self.id] < MAX_CONSECUTIVE_TIMES)
                    {
                        times_ate_mut[self.id] = times_ate_mut[self.id] + 1;
                        states_mut[self.id] = true;
                        fork.get(self.left_fork()).unwrap().acquire();
                        println!("Filosofo {} tomando palito {} ", self.id, self.left_fork());
                        fork.get(self.right_fork()).unwrap().acquire();
                        println!("Filosofo {} tomando palito {} ", self.id, self.right_fork());

                        println!("Filosofo {} comiendo!", self.id);
                    }

                    if(!times_ate_mut.contains(&0))
                    {
                        for item in &mut *times_ate_mut { *item = 0; }
                    }
                }
            }

            thread::sleep(Duration::from_millis(thread_rng().gen_range(1000, 5000)));

            if let Ok(mut states_mut) = eating_states.write()
            {
                if (states_mut[self.id] == true)
                {
                    println!("{:?}", *states_mut);
                    fork.get(self.left_fork()).unwrap().release();
                    println!("Filosofo {} soltó palito {} ", self.id, self.left_fork());
                    fork.get(self.right_fork()).unwrap().release();
                    println!("Filosofo {} soltó palito {} ", self.id, self.right_fork());
                    states_mut[self.id] = false;

                    println!("Filosofo {} pensando!", self.id);
                }
            }
        }
    }
}
fn main() {

    let eating_states = Arc::new(RwLock::new(vec!(false, false, false, false, false)));
    let times_philosopher_ate = Arc::new(RwLock::new(vec!(0,0,0,0,0)));

    let fork = Arc::new(vec![
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
        Semaphore::new(1),
    ]);

    //let mut handles = vec![];

    let philosophers = vec![
        Philosopher::new("Rojo", 0),
        Philosopher::new("Azul", 1),
        Philosopher::new("Amarillo", 2),
        Philosopher::new("Violeta", 3),
        Philosopher::new("Verde", 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map( |philosopher| {
        let fork_clone = Arc::clone(&fork);
        let eating_states_clone = eating_states.clone();
        let times_philosopher_ate_clone = times_philosopher_ate.clone();

        thread::spawn(move || {
            philosopher.eat(fork_clone, eating_states_clone, times_philosopher_ate_clone);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

