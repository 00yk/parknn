use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use rayon::prelude::*;

#[derive(Clone)]
struct DataPoint {
    label: Option<u8>,
    features: Vec<f32>,
}

fn read_csv(file: &Path, is_train: bool) -> Vec<DataPoint> {
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

fn read_csv_opt(file: &Path, is_train: bool) -> Vec<DataPoint> {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .par_lines()
        .map(|line| {
            // let line = line.unwrap();
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
    // let orig = dists.clone();
    let mut indexes = (0..dists.len()).collect::<Vec<_>>();
    // indexes.sort_by(|&a, &b| dists[a].partial_cmp(&dists[b]).unwrap());
    // dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
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

    // training
    //     .iter()
    //     // find element of `training` with the smallest distance_sqr to `pixel`
    //     .min_by_key(|p| NotNan::new(
    //         distance_sqr(p.features.as_slice(), features)).unwrap())
    //     .unwrap()
    //     .label
}

fn main() {
    // let mut f = File::open("./train5.csv").unwrap();
    // let mut contents = String::new();
    // f.read_to_string(&mut contents).unwrap();
    // // println!("{}", contents);
    // let lengths: Vec<_> = "hello world\nfizbuzz"
    //     .par_lines()
    //     .map(|l| l.len())
    //     .collect();
    // contents.par_lines().for_each(|l| {
    //     println!("{}", l);
    // });
    // println!("{:?}", lengths);
    // BufReader::new(File::open("./train5.csv").unwrap())
    //     .lines().for_each(|l| {
    //         println!("{}", l.unwrap());
    //     })
    // println!("{}", rayon::current_num_threads()); let train_set = read_csv(Path::new("./susy100K.csv"), true);
    let train_set = read_csv_opt(Path::new("./susy100K.csv"), true);
    let test_set = read_csv_opt(Path::new("./susytest1000.csv"), false);
    test_set.par_iter()
            .for_each(|test_sample| {
                classify(train_set.as_slice(),test_sample.features.as_slice());
            });

    // let mut stop_iter = 20;
    // let NUM_CHUNKS = 4;
    // let chunk_size = (test_set.len() + NUM_CHUNKS - 1) / NUM_CHUNKS;
    // let mut handles: Vec<_> = vec![];
    // for i in 0..NUM_CHUNKS {
    //     let lo = i * chunk_size;
    //     let hi = std::cmp::min(lo + chunk_size, test_set.len());
    //     let handle = thread::spawn(|| {
    //         for test_sample in &test_set[lo..hi] {
    //             classify(train_set.as_slice(), test_sample.features.as_slice());
    //         }
    //     });
    //     handles.push(handle);
    // }
    // for i in handles {
    //     i.join();
    // }
    // for test_sample in test_set {
    //     classify(train_set.as_slice(), test_sample.features.as_slice());
    //     // println!("{}", label.unwrap());
    //     // stop_iter -= 1;
    //     // if stop_iter == 0 {
    //     //     break;
    //     // }
    // }

}
