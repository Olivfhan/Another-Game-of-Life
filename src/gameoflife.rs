//use std::io;

const MAX_SIZE: usize = 31;
const SIZE: usize = 32;


pub fn evolve(cells: [u32;SIZE]) -> [u32;SIZE] {
    
    let mut copy: [u32;SIZE] = [0;SIZE];
    let mut i = 0;
    let mut j = MAX_SIZE;
    let mut counter: u32 = 0;
    
    while i<(MAX_SIZE+1) {
        loop {

            if i==0 {
                if j==MAX_SIZE {
                    counter = counter + ((cells[i] >> (j-1)) & 1) + ((cells[i+1] >> j) & 1) + ((cells[i+1] >> (j-1)) & 1);
                }
                else if j==0 {
                    counter = counter + ((cells[i] >> (j+1)) & 1) + ((cells[i+1] >> j) & 1) + ((cells[i+1] >> (j+1)) & 1);
                }
                else {
                    counter = counter + ((cells[i] >> (j+1)) & 1) + ((cells[i] >> (j-1)) & 1) + ((cells[i+1] >> j) & 1) + ((cells[i+1] >> (j+1)) & 1) + ((cells[i+1] >> (j-1)) & 1);
                }
            }
            else if i==MAX_SIZE {
                if j==MAX_SIZE {
                    counter = counter + ((cells[i] >> (j-1)) & 1) + ((cells[i-1] >> j) & 1) + ((cells[i-1] >> (j-1)) & 1);
                }
                else if j==0 {
                    counter = counter + ((cells[i] >> (j+1)) & 1) + ((cells[i-1] >> j) & 1) + ((cells[i-1] >> (j+1)) & 1);
                }
                else {
                    counter = counter + ((cells[i] >> (j+1)) & 1) + ((cells[i] >> (j-1)) & 1) + ((cells[i-1] >> j) & 1) + ((cells[i-1] >> (j+1)) & 1) + ((cells[i-1] >> (j-1)) & 1);
                }
            } else {
                if j==MAX_SIZE {
                    counter = counter + ((cells[i] >> (j-1)) & 1) + ((cells[i+1] >> j) & 1) + ((cells[i+1] >> (j-1)) & 1) + ((cells[i-1] >> j) & 1) + ((cells[i-1] >> (j-1)) & 1);
                }
                else if j==0 {
                    counter = counter + ((cells[i] >> (j+1)) & 1) + ((cells[i+1] >> j) & 1) + ((cells[i+1] >> (j+1)) & 1) + ((cells[i-1] >> j) & 1) + ((cells[i-1] >> (j+1)) & 1);
                }
                else {
                    counter = counter + ((cells[i] >> (j+1)) & 1) + ((cells[i] >> (j-1)) & 1) + ((cells[i+1] >> j) & 1) + ((cells[i+1] >> (j+1)) & 1) + ((cells[i+1] >> (j-1)) & 1);
                    counter = counter + ((cells[i-1] >> j) & 1) + ((cells[i-1] >> (j+1)) & 1) + ((cells[i-1] >> (j-1)) & 1);
                } }
            if counter==3{
                copy[i] = copy[i] | (1 << j);
            }
            else if counter==2 {
                copy[i] = copy[i] | (((cells[i] >> j) & 1) << j);
            }

            counter = 0;

            if j==0 {
                break; 
            }

            j = j-1;
        }
            i = i+1;
            j = MAX_SIZE;
    }
    return copy

}













