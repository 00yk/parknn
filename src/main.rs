use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rayon::prelude::*;

#[derive(Clone)]
struct DataPoint {
    label: Option<u8>,
    features: Vec<f32>,
}

fn read_csv_opt(file: &Path, is_train: bool) -> Vec<DataPoint> {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .par_lines()
        .map(|line| {
            let mut iter = line.trim().split(',');
            let first = if is_train {
                Some(iter.next().unwrap().parse::<u8>().unwrap())
            } else {
                None
            };
            let rest = iter.map(|x| x.parse::<f32>().unwrap());

            DataPoint {
                label: first,
                features: rest.collect(),
            }
        })
        .collect()
}

fn distance_sqr(x: &[f32], y: &[f32]) -> f32 {
    x.iter()
        .zip(y.iter())
        .fold(0.0, |s, (&a, &b)| s + (a - b) * (a - b))
}


fn classify(training: &[DataPoint], features: &[f32]) -> Option<u8> {
    let mut dists: Vec<f32> = vec![];
    for train_sample in training {
        let dist = distance_sqr(train_sample.features.as_slice(), features);
        dists.push(dist);

    }
    let mut indexes = (0..dists.len()).collect::<Vec<_>>();
    // indexes.sort_by(|&a, &b| dists[a].partial_cmp(&dists[b]).unwrap());
    let k = 7;
    let mut ones = 0;
    let mut zeros = 0;
    for i in 0..k {
        pdqselect::select_by(&mut indexes, i+1,
                         |&a, &b| dists[a].partial_cmp(&dists[b]).unwrap());
        let l = training[indexes[i]].label.unwrap();
        if l == 1 {
            ones += 1
        } else {
            zeros += 1
        }
    }
    if ones > zeros {
        // println!("{}", 1);
        return Some(1)
    } else {
        // println!("{}", 0);
        return Some(0)
    }

}

fn main() {
    let train_set = read_csv_opt(Path::new("./susy100K.csv"), true);
    let test_set = read_csv_opt(Path::new("./susytest1000.csv"), false);
    let r = test_set.par_iter()
            .map(|test_sample| {
                classify(train_set.as_slice(),test_sample.features.as_slice()).unwrap()
            }).collect::<Vec<_>>();
    println!("{:?}", r);
}
