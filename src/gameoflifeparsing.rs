use std::io;


const MAX_SIZE: usize = 31;
const SIZE: usize = 32;

const WHITE_SQUARE: &str= "■ ";
const BLACK_SQUARE: &str = "□ ";


pub fn format_initial() -> [u32;SIZE] {
    let mut counter = 0;
    let mut j = MAX_SIZE;
    let mut res: [u32;SIZE] = [0;SIZE];
    let mut current: String;
    println!("Enter the next strings or numbers for any of the predetermined grids:\npulsar (probably the most interesting given the limitations of the current version): 1\nglider_gun: 2\ncube: 3");
    while counter<(MAX_SIZE+1) { 
        current = String::new();
        io::stdin()
        .read_line(&mut current)
        .expect("Failed to read line");
        let mut ptr = current.as_mut_ptr();
        if current.trim() == "pulsar" || current.trim() == "1" {
            return [0,0,7280,0,17028,17028,17028,7280,0,7280,17028,17028,17028,0,7280,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        } else if current.trim() == "zero" {
            j = usize::MAX;
        } else if current.trim() == "glider_gun" || current.trim() == "2" {
            return [0,64,320,394752,558592,1611679232,1611751744,1065024,557056,393216,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        } else if current.trim() == "cube" || current.trim() == "3" {
            return [0,0,0,0,0,0,0,0,0,0,0,0,0,3244032,3244032,0,3244032 ,3244032 ,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        }
        loop {
            if j==usize::MAX {
                break;
            }
            if unsafe {*ptr} == 49 {
                res[counter] = res[counter] | (1 << j);
            }
            ptr = unsafe { ptr.offset(1) };
            if j==0 {
                break;
            }
            j-=1;
        }
        j=MAX_SIZE;
        counter+=1;
    
    }
    return res;
}


pub fn output_to_terminal(cells: [u32;SIZE]) {
    let mut j = MAX_SIZE;
    let mut current_row = String::new();
    let mut res = String::new();
    for row in cells {
        loop {
            if ((row >> j) & 1)==1 {
                current_row = current_row + WHITE_SQUARE;
            }
            else {
                current_row = current_row + BLACK_SQUARE;
            }
            if j==0 {
                break;
            }
            j = j-1;
        }
        j = MAX_SIZE;
        res = res + &current_row + "\n";
        current_row = String::new();
    }
    println!("{res}");
    return
}
