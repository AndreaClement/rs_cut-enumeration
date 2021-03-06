// => compute primary inputs and gates
// => compute fanins of all gates
// ==> can easily run cut_enumeration from there

use std::fs;
type Cut_t = Vec<u16>;

fn main() {
    let contents = fs::read_to_string("test.txt")
        .expect("Something went wrong reading the file");


    let NTK            = convert_to_table(contents.split('\n').collect());
    let (PI, GATES)    = find_PI_and_GATES(&NTK);

    for node in &NTK {
        println!("{:?}", node);
    }
    println!("\n");

    println!("{:?}", PI);
    println!("{:?}", GATES);
    println!("\n");

    build_cuts(&NTK, &PI, &GATES, 5);

}

fn convert_to_table(split: Vec<&str>) -> Vec<Vec<u16>> {
    let mut ntk = Vec::with_capacity(split.len());
    for i in 0..(split.len() - 1) {

        let mut fanins: Vec<u16> = Vec::new();
        for elem in split[i].split(',') {
            if elem.len() > 0 {
                fanins.push(elem.parse().unwrap());
            }
        }

        ntk.push(fanins);
    }

    return ntk;
}

fn find_PI_and_GATES(NTK: &Vec<Vec<u16>>) -> (Vec<u16>, Vec<u16>) {
    let mut primary_inputs = Vec::new();
    let mut gates          = Vec::new();

    for i in 0..(NTK.len()) {
        if NTK[i].is_empty() {
            primary_inputs.push(i as u16);
        } else {
            gates.push(i as u16);
        }
    }

    return (primary_inputs, gates);
}

fn dominates(cut1: &Cut_t, cut2: &Cut_t) -> bool {
    // AKA cut1 dominates if it belongs to cut2
    if cut1.len() <= cut2.len() {
        for i in cut1 {
            if !(cut2.contains(&i)) {
                return false;
            }
        }
        return true;
    }
    return false;
}


fn is_dominated(cut: &Cut_t, set_of_cuts: &Vec<Cut_t>) -> bool {
    for cut1 in set_of_cuts {
        if dominates(cut1, cut) {
            return true;
        }
    }
    return false;
}

fn add_and_remove_dominated(cut: &Cut_t, set_of_cuts: &mut Vec<Cut_t>) {
    let mut new_set = Vec::new();
    new_set.push(cut.clone());
    for i in 0..set_of_cuts.len() {
        if !dominates(cut, &set_of_cuts[i]) {
            new_set.push(set_of_cuts[i].clone());
        }
    }
    *set_of_cuts = new_set.clone();
}

fn union(a: &Cut_t, b: &Cut_t) -> Cut_t {
    let mut ret = a.clone();
    let step_b = b.clone();
    for i in step_b {
        if !ret.contains(&i) {
            ret.push(i);
        }
    }

    return ret;
}

fn build_cuts(NTK: &Vec<Vec<u16>>, PI: &Vec<u16>, GATES: &Vec<u16>, k: usize) {
    let mut CUTS: Vec<Vec<Cut_t>> = vec![Vec::new(); NTK.len()];

    struct Rec<'s> {
        f: &'s dyn Fn(&Rec, usize, usize, Cut_t, usize, &mut  Vec<Vec<Cut_t>>)
    }

    let rec = Rec {
        f: &|rec: &Rec, nmb, end, curr: Cut_t, index, cuts| {
            if nmb == end {
                if curr.len() <= k && !is_dominated(&curr, &cuts[index]) {
                    add_and_remove_dominated(&curr, &mut cuts[index]);
                }
            } else {
                let my_fanins: Cut_t = NTK[index].clone();

                let fanin: usize = usize::from(my_fanins[nmb]);
                let fanin_cuts = cuts[fanin].clone();

                for cut in fanin_cuts {
                    (rec.f)(&rec, nmb + 1, end, union(&curr, &cut), index, cuts);
                }
            }
        }
    };

    for i in PI {
        let j = usize::from(*i);
        let mut init = Vec::new();
        init.push(*i);
        CUTS[j].push(init.clone());
    }

    for i in GATES {
        let j = usize::from(*i);

        (rec.f)(&rec, 0, NTK[j].len(), Vec::new(), j, &mut CUTS);

        let mut me: Cut_t = Vec::new();
        me.push(*i);
        CUTS[j].push(me.clone());
    }

    println!("CUTS :");
    for set in CUTS {
        for cut in set {
            println!("{:?}", cut);
        }
        println!("");
    }
}
