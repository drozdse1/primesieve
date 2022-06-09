use bitvec::prelude::*;
use std::mem;
use num_integer::Roots;
use std::thread;
use std::time::Duration;

const MAX_SIZE_SIEVE: usize = 10_000_000_000;
const MAX_SIZE_SIEVE_SQRT: usize = 10_000;
const NUMBER_OF_THREADS: usize = 2;

pub struct SingleThreadSieve {
    sieve: Box<[bool]>,    
    sieve_compact: BitBox
}

pub struct MultiThreadSieve {

    /*
     *      Example with 2 threads:
     * 
     *     ┌──────────────┐ 0
     *     │ Box<[bool]>T0│
     *     ├──────────────┤ MAX_SIZE_SIEVE / 2 (number of threads)
     *     │ Box<[bool]>T1│
     *     └──────────────┘ MAX_SIZE_SIEVE
     *
     *     With 2 threads we have to sieves for diffrent ranges s0 (sieve 0) 
     *     is responsible from 0 to MAX_SIZE_SIZEVE / 2 - 1
     *     and s1 is responsible from MAX_SIZE_SIEVE / 2 to MAX_SIZE_SIEVE - 1
     * 
     */

    sieve: Box<[[bool; MAX_SIZE_SIEVE / NUMBER_OF_THREADS]; NUMBER_OF_THREADS]>
}

impl SingleThreadSieve {
    pub fn init() -> SingleThreadSieve {
        let ps: SingleThreadSieve = Self {
            sieve_compact : bitvec![0; MAX_SIZE_SIEVE / 1_000].into_boxed_bitslice(),
            sieve: vec![false;  MAX_SIZE_SIEVE].into_boxed_slice()
        };
        return ps;
    }

    pub fn calc(&mut self) {
        for n in 2..MAX_SIZE_SIEVE.nth_root(2){
           if self.sieve[n]==false {
               let mut c:usize = n;
               println!("{}", c);
               while c < MAX_SIZE_SIEVE {
                    self.sieve[c]=true;
                    c += n;
               }
           }
        }        
    }
}
