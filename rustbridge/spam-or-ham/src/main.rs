extern crate hyper;
extern crate zip;
extern crate rustlearn;
extern crate time;

use std::io::{Cursor, Read};

use hyper::client::Client;
use zip::read::ZipArchive;
use rustlearn::prelude::*;
use rustlearn::feature_extraction::DictVectorizer;
use rustlearn::cross_validation::CrossValidation;
use rustlearn::linear_models::sgdclassifier;
use rustlearn::metrics::accuracy_score;

fn download(url: &str) -> Vec<u8> {
    let client = Client::new();
    let mut response = client.get(url).send().unwrap();

    let mut data = Vec::new();
    response.read_to_end(&mut data).unwrap();

    data
}

fn unzip(zipped: Vec<u8>) -> String {
    let mut archive = ZipArchive::new(Cursor::new(zipped)).unwrap();
    let mut file = archive.by_name("SMSSpamCollection").unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}

fn parse(data: &str) -> (SparseRowArray, Array) {
    // initialize the vectorizer
    let mut vectorizer = DictVectorizer::new();
    let mut labels = Vec::new();

    // iterate over pairs of (row_number, line)
    for (row_num, line) in data.lines().enumerate() {
        // the label and text message are separated by a tab, split the line in two
        let (label, text) = line.split_at(line.find('\t').unwrap());

        // convert the label to binary
        labels.push(match label {
            "spam"  => 0.0,
            "ham"   => 1.0,
            _       => panic!(format!("Invalid label: {}", label))
        });

        // vectorizer will keep a mapping from tokens to column indices
        for token in text.split_whitespace() {
            vectorizer.partial_fit(row_num, token, 1.0);
        }
    }

    (vectorizer.transform(), Array::from(labels))
}

fn fit(x: &SparseRowArray, y: &Array) -> (f32, f32) {
    let num_epochs = 10;
    let num_folds = 10;

    let mut test_accuracy = 0.0;
    let mut train_accuracy = 0.0;

    for (train_indices, test_indices) in CrossValidation::new(y.rows(), num_folds) {
        // slice the feature matrices
        let x_train = x.get_rows(&train_indices);
        let x_test = x.get_rows(&test_indices);

        // slice the target vectors
        let y_train = y.get_rows(&train_indices);
        let y_test = y.get_rows(&test_indices);

        let mut model = sgdclassifier::Hyperparameters::new(x.cols())
            .learning_rate(0.05)
            .l2_penalty(0.01)
            .build();

        for _ in 0..num_epochs {
            model.fit(&x_train, &y_train).unwrap();
        }

        let fold_test_accuracy = accuracy_score(&y_test, &model.predict(&x_test).unwrap());
        let fold_train_accuracy = accuracy_score(&y_train, &model.predict(&x_train).unwrap());

        test_accuracy += fold_test_accuracy;
        train_accuracy += fold_train_accuracy;
    }

    (test_accuracy / num_folds as f32,
    train_accuracy / num_folds as f32)
}

fn main() {
    let zipped = download("https://archive.ics.uci.\
                           edu/ml/machine-learning-databases/00228/smsspamcollection.zip");
    println!("Downloaded {} bytes of data", zipped.len());

    let raw_data = unzip(zipped);

    for line in raw_data.lines().take(5) {
        println!("{}", line);
    }

    let (x, y) = parse(&raw_data);

    println!("X: {} rows, {} columns, {} non-zero entries. Y: {:.2}% positive class",
            x.rows(), x.cols(), x.nnz(), y.mean() * 100.0);

    let start_time = time::precise_time_ns();
    let (test_accuracy, train_accuracy) = fit(&x, &y);
    let duration = time::precise_time_ns() - start_time;

    println!("Training time: {:.3} seconds", duration as f64 / 1.0e+9);
    println!("Test accuracy: {:.3}, train accuracy: {:.3}", test_accuracy, train_accuracy);
}
