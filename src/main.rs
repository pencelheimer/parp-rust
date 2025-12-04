#![allow(unused_variables)]
#![allow(deprecated)]

use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

fn test_1() {
    let n = 100_000_000;

    // Послідовна версія
    let start = Instant::now();
    let seq_result: i128 = (0..n).into_iter().map(|x| x * x).sum();
    let seq_time = start.elapsed();

    // Паралельна версія
    let start = Instant::now();
    let par_result: i128 = (0..n).into_par_iter().map(|x| x * x).sum();
    let par_time = start.elapsed();

    println!("Sequential: {} in {:?}", seq_result, seq_time);
    println!("Parallel: {} in {:?}", par_result, par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );
}

fn test_2() {
    let size = 10_000_000;
    let mut rng = rand::thread_rng();

    // Створення випадкового масиву
    let mut seq_data: Vec<i32> = (0..size).map(|_| rng.gen_range(0..1000000)).collect();
    let mut par_data = seq_data.clone();

    // Послідовне сортування
    let start = Instant::now();
    seq_data.sort();
    let seq_time = start.elapsed();

    // Паралельне сортування
    let start = Instant::now();
    par_data.par_sort();
    let par_time = start.elapsed();

    println!("Sequential sort: {:?}", seq_time);
    println!("Parallel sort: {:?}", par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );

    // Перевірка коректності
    assert_eq!(seq_data, par_data);
}

fn test_3() {
    type Matrix = Vec<Vec<f64>>;

    fn create_matrix(rows: usize, cols: usize, value: f64) -> Matrix {
        vec![vec![value; cols]; rows]
    }

    fn sequential_matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
        let n = a.len();
        let m = b[0].len();
        let p = b.len();
        let mut result = create_matrix(n, m, 0.0);
        for i in 0..n {
            for j in 0..m {
                for k in 0..p {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }

    fn parallel_matrix_multiply(a: &Matrix, b: &Matrix) -> Matrix {
        let n = a.len();
        let m = b[0].len();
        let p = b.len();

        let mut result = create_matrix(n, m, 0.0);

        result.par_iter_mut().enumerate().for_each(|(i, row)| {
            for j in 0..m {
                for k in 0..p {
                    row[j] += a[i][k] * b[k][j];
                }
            }
        });

        result
    }

    let size = 512;
    let a = create_matrix(size, size, 1.5);
    let b = create_matrix(size, size, 2.0);

    // Послідовна версія
    let start = Instant::now();
    let seq_result = sequential_matrix_multiply(&a, &b);
    let seq_time = start.elapsed();

    // Паралельна версія
    let start = Instant::now();
    let par_result = parallel_matrix_multiply(&a, &b);
    let par_time = start.elapsed();

    println!("Matrix size: {}x{}", size, size);
    println!("Sequential: {:?}", seq_time);
    println!("Parallel: {:?}", par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );

    // Перевірка коректності (порівняння перших елементів)
    assert!((seq_result[0][0] - par_result[0][0]).abs() < 0.001);
}

fn test_4() {
    struct Image {
        width: usize,
        height: usize,
        data: Vec<u8>,
    }

    impl Image {
        fn new(width: usize, height: usize) -> Self {
            Image {
                width,
                height,
                data: vec![128; width * height * 3],
            }
        }
        fn get_pixel(&self, x: usize, y: usize) -> [u8; 3] {
            let idx = (y * self.width + x) * 3;
            [self.data[idx], self.data[idx + 1], self.data[idx + 2]]
        }
        fn set_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
            let idx = (y * self.width + x) * 3;
            self.data[idx] = color[0];
            self.data[idx + 1] = color[1];
            self.data[idx + 2] = color[2];
        }
    }

    fn apply_filter_sequential(img: &Image) -> Image {
        let mut result = Image::new(img.width, img.height);
        for y in 1..img.height - 1 {
            for x in 1..img.width - 1 {
                let mut sum = [0u16; 3];
                // Blur 3x3
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let pixel =
                            img.get_pixel((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                        sum[0] += pixel[0] as u16;
                        sum[1] += pixel[1] as u16;
                        sum[2] += pixel[2] as u16;
                    }
                }
                result.set_pixel(
                    x,
                    y,
                    [(sum[0] / 9) as u8, (sum[1] / 9) as u8, (sum[2] / 9) as u8],
                );
            }
        }
        result
    }

    fn apply_filter_parallel(img: &Image) -> Image {
        let mut result = Image::new(img.width, img.height);
        result
            .data
            .par_chunks_mut(img.width * 3)
            .enumerate()
            .skip(1)
            .take(img.height - 2)
            .for_each(|(y, row)| {
                for x in 1..img.width - 1 {
                    let mut sum = [0u16; 3];
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let pixel =
                                img.get_pixel((x as i32 + dx) as usize, (y as i32 + dy) as usize);
                            sum[0] += pixel[0] as u16;
                            sum[1] += pixel[1] as u16;
                            sum[2] += pixel[2] as u16;
                        }
                    }
                    let idx = x * 3;
                    row[idx] = (sum[0] / 9) as u8;
                    row[idx + 1] = (sum[1] / 9) as u8;
                    row[idx + 2] = (sum[2] / 9) as u8;
                }
            });
        result
    }

    let img = Image::new(4096, 4096);

    // Послідовна версія
    let start = Instant::now();
    let seq_result = apply_filter_sequential(&img);
    let seq_time = start.elapsed();

    // Паралельна версія
    let start = Instant::now();
    let par_result = apply_filter_parallel(&img);
    let par_time = start.elapsed();

    println!("Image size: {}x{}", img.width, img.height);
    println!("Sequential: {:?}", seq_time);
    println!("Parallel: {:?}", par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );
}

fn test_5() {
    fn count_words_sequential(text: &str) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        for word in text.split_whitespace() {
            let word = word.to_lowercase();
            *counts.entry(word).or_insert(0) += 1;
        }
        counts
    }

    fn count_words_parallel(text: &str) -> HashMap<String, usize> {
        text.par_split_whitespace()
            .map(|word| {
                let word = word.to_lowercase();
                let mut map = HashMap::new();
                map.insert(word, 1);
                map
            })
            .reduce(HashMap::new, |mut a, b| {
                for (key, value) in b {
                    *a.entry(key).or_insert(0) += value;
                }
                a
            })
    }

    // Генерація тестового тексту
    let words = vec!["hello", "world", "rust", "rayon", "parallel"];
    let text: String = (0..1_000_000)
        .map(|i| words[i % words.len()])
        .collect::<Vec<_>>()
        .join(" ");

    // Послідовна версія
    let start = Instant::now();
    let seq_counts = count_words_sequential(&text);
    let seq_time = start.elapsed();

    // Паралельна версія
    let start = Instant::now();
    let par_counts = count_words_parallel(&text);
    let par_time = start.elapsed();

    println!("Text length: {} words", text.split_whitespace().count());
    println!("Unique words: {}", seq_counts.len());
    println!("Sequential: {:?}", seq_time);
    println!("Parallel: {:?}", par_time);
    println!(
        "Speedup: {:.2}x",
        seq_time.as_secs_f64() / par_time.as_secs_f64()
    );

    // Перевірка коректності
    assert_eq!(seq_counts.len(), par_counts.len());
}

fn main() {
    test_1();

    println!();
    test_2();

    println!();
    test_3();

    println!();
    test_4();

    println!();
    test_5();
}
