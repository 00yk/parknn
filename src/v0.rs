use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use ordered_float::NotNan;

struct DataPoint {
    label: Option<u8>,
    features: Vec<f32>,
}

fn read_csv(file: &Path, is_train: bool) -> Vec<DataPoint> {
    // let f = File::open(file).unwrap();
    // let reader = BufReader::new(f);
    // let lines = reader.lines();
    // println!("{:?}", lines);
    // let mut ret: Vec<DataPoint> = vec![];
    // let mut stop_iter_num = 1;
    // for line in lines {
    //     let l = line.unwrap();
    //     // println!("{:?}", l);
    //     let mut iter = l.as_str().trim().split(',');
    //     let first = if is_train {
    //         Some(iter.next().unwrap().parse::<u8>().unwrap())
    //     } else {
    //         None
    //     };
    //     let rest = iter.map(|x| x.parse::<f32>().unwrap());

    //     // println!("{:?}", first);
    //     // println!("{:?}", iter);
    //     // for i in rest {
    //     //     println!("{}", i);
    //     // }
    //     ret.push(DataPoint {
    //         label: first,
    //         features: rest.collect()
    //     });

    //     stop_iter_num -= 1;
    //     if stop_iter_num == 0 {
    //         break;
    //     }
    // }
    // for ele in &ret {
    //     println!("{:?}", ele.label);
    //     println!("{:?}", ele.features);
    // }
    // println!("Finish");
    // ret
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut iter = line.as_str().trim().split(',');
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
    training
        .iter()
        // find element of `training` with the smallest distance_sqr to `pixel`
        .min_by_key(|p| NotNan::new(
            distance_sqr(p.features.as_slice(), features)).unwrap())
        .unwrap()
        .label
}

fn main() {
    // let res = distance_sqr(&[1, 2, 3], &[3, 4, 5]);
    // println!("{}", res);
    let mut i = 0;
    // let r = read_csv(Path::new("./train.csv"), true);
    let train_set = read_csv(Path::new("./susy100K.csv"), true);
    let test_set = read_csv(Path::new("./susytest1000.csv"), false);
    for test_sample in test_set {
        // println!("{:?}", test_sample.label);
        let label = classify(train_set.as_slice(), test_sample.features.as_slice());
        // println!("{}", label.unwrap());
    }
        
    // test_set
    //     .iter()
    //     .map(|x| classify(train_set.as_slice(),
    //                       x.features.as_slice()).unwrap());
    // println!("{}", r.iter().count());

    // for l in read_csv(Path::new("susytest1000.csv")) {
    //     println!("{}", l);
    //     i += 1;
    //     if i == 10 {
    //         break;
    //     }
    // }
}
