// => compute primary inputs and gates
// => compute fanins of all gates
// ==> can easily run cut_enumeration from there

use std::fs;
//use std::env;

fn main() {
    let contents = fs::read_to_string("test.txt")
        .expect("Something went wrong reading the file");


    let NTK            = convert_to_table(contents.split('\n').collect());
    let (PI, GATES)    = find_PI_and_GATES(&NTK);

    for node in NTK {
        println!("{:?}", node);
    }
    println!("\n");

    println!("{:?}", PI);
    println!("{:?}", GATES);

}

fn convert_to_table(split: Vec<&str>) -> Vec<Vec<i64>> {
    let mut ntk = Vec::with_capacity(split.len());
    for i in 0..(split.len() - 1) {
        let mut fanins: Vec<i64> = Vec::new();
        for elem in split[i].split(',') {
            if elem.len() > 0 {
                fanins.push(elem.parse().unwrap());
            }
        }

        ntk.push(fanins);
    }

    return ntk;
}

fn find_PI_and_GATES(NTK: &Vec<Vec<i64>>) -> (Vec<i64>, Vec<i64>) {
    let mut primary_inputs = Vec::new();
    let mut gates          = Vec::new();

    for i in 0..(NTK.len() - 1) {
        if NTK[i].is_empty() {
            primary_inputs.push(i as i64);
        } else {
            gates.push(i as i64);
        }
    }

    return (primary_inputs, gates);
}
