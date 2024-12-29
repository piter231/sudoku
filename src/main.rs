use std::io::{self, Write};

const DIM : usize = 9;
//must  be divisible by 3
const CHUNK_SIZE : usize = DIM / 3;

fn count_possible(tab : [bool; DIM+1]) -> usize{
    let mut out:usize=0;
    for i in 1..DIM+1{
        if tab[i] {
            out +=1;
        }
    }
    return out;
}

fn get_possible(tab : [bool; DIM+1]) -> usize{
    for i in 1..DIM+1{
        if tab[i] {
            return i;
        }
    }
    return 0;
}
fn create_with_only_one(k : usize) ->  [bool; DIM+1]{
    let mut out : [bool; DIM+1] = [false; DIM+1];
    out[k]=true;
    return out;
}


fn create_possible_count(possible : [[[bool; DIM+1]; DIM]; DIM] ) -> [Vec<(usize, usize)>; DIM+1]{
    let mut possible_count : [Vec<(usize, usize)>; DIM+1] = Default::default();
    for i in 0..DIM{
        for j in 0..DIM{
            possible_count[count_possible(possible[i][j])].push((i, j));
        }
    }
    return possible_count;
}

fn _print_possible(possible : [[[bool; DIM+1]; DIM]; DIM]){
    for i in 0..DIM{
        for j in 0..DIM{
            print!("({}, {}): ", i,j);
            for k in 1..DIM+1{
                print!("{} ", possible[i][j][k]);
            }
            println!();
        }
    }
    println!();
}

fn print_sudoku_from_possible(possible : [[[bool; DIM+1]; DIM]; DIM]){
    for i in 0..DIM{
        for j in 0..DIM{
            if count_possible(possible[i][j])!=1 {
                print!("x");
            }
            else{
                print!("{}",get_possible(possible[i][j]))
            }
        }
        println!();
    }
    println!();
}

fn solve(mut possible : [[[bool; DIM+1]; DIM]; DIM] ){
    let mut possible_count = create_possible_count(possible);

    if possible_count[0].len()>0{
        println!("impossible in this instance");
        return;
    }
    let mut done_prev=0;
    let mut done_curr = possible_count[1].len();
    if done_curr==DIM*DIM{
        println!("Found solution:");
        print_sudoku_from_possible(possible);
        return;
    }

    while done_prev<done_curr {

        for i in 0..DIM{
            for j in 0..DIM{
                let number_of_possibilities=count_possible(possible[i][j]);
                if number_of_possibilities==0{
                    //println!("impossible in this instance");
                    return;
                }
                else if number_of_possibilities==1{
                    for k in 0..DIM{
                        if k!=i {
                            possible[k][j][get_possible(possible[i][j])]=false;
                        }
                        if k!=j {
                            possible[i][k][get_possible(possible[i][j])]=false;
                        }
                    }
                    let ii = i/CHUNK_SIZE*CHUNK_SIZE;
                    let jj = j/CHUNK_SIZE*CHUNK_SIZE;
                    for x in ii..ii+CHUNK_SIZE{
                        for y in jj..jj+CHUNK_SIZE{
                            if x!=i && y!=j{
                                possible[x][y][get_possible(possible[i][j])] = false;
                            }
                        }
                    }
                }
            }
        }
        possible_count=create_possible_count(possible);
        done_prev=done_curr;
        done_curr=possible_count[1].len();

        if possible_count[0].len()>0{
            //println!("impossible in this instance");
            return;
        }
        /*
       print_possible(possible);
       print_sudoku_from_possible(possible);
       for xd in possible_count{
            print!("{} ", xd.len());
       }
       println!();
       */
    }
    if done_curr==DIM*DIM{
        solve(possible);
    }
    else{
        for k in 2..DIM+1{
            if possible_count[k].len()>0 {
                for (i,j) in possible_count[k].clone(){
                    let old_array=possible[i][j];
                    for d in 1..DIM+1{
                        if old_array[d]{
                            possible[i][j]=create_with_only_one(d);
                            solve(possible);
                        }
                    }
                    break;
                }
                break;
            }
        }
    }

}

fn main(){

    let mut matrix = vec![vec![-1 as i8; DIM]; DIM];
    let mut possible : [[[bool; DIM+1]; DIM]; DIM] = [[[true;DIM+1]; DIM]; DIM];
    println!("Enter the sudoku: (without spaces, rows separated by newline)");
    for i in 0..DIM{
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for j in 0..DIM{
            let k = input.trim().chars().nth(j).expect("Index out of bounds");
            if k.is_digit(10) {
                matrix[i][j] = k.to_digit(10).unwrap() as i8;
            }
        }
    }

    for i in 0..DIM{
        for j in 0..DIM{
            if matrix[i][j]!=-1 {
                for k in 1..DIM+1{
                    if k as i8 != matrix[i][j]{
                        possible[i][j][k]=false;
                    }
                }
            }
        }
    }
    
    solve(possible);
}
