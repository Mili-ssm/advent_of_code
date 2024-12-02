use advent_of_code::advent_2024::{day_01, day_02};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rand::distributions::Uniform;
use rand::{Rng, thread_rng};

pub fn day_01(c: &mut Criterion) {
    //Load Input
    c.bench_function("Day_01/Loader", |b| b.iter(|| day_01::loader()));

    //Result
    let (list_a, list_b) = day_01::loader();
    println!("Day 01 | Part 1: {:?}", day_01::part_1(&list_a, &list_b));
    println!(
        "Day 01 | Part 2: {:?}",
        day_01::part_2_dict(&list_a, &list_b)
    );

    //Part 1
    c.bench_function("Day_01/Part_1", |b| {
        b.iter(|| day_01::part_1(&list_a, &list_b))
    });

    //Part 2
    let mut group = c.benchmark_group("Day_01/Part_2");
    group.bench_function("Dict", |b| b.iter(|| day_01::part_2_dict(&list_a, &list_b)));
    group.bench_function("Vec", |b| b.iter(|| day_01::part_2_vec(&list_a, &list_b)));

    let mut rng = thread_rng();

    const MAX_ELEMENTS: usize = 1_000_000;
    const MAX_ITERS: u32 = 7;
    let rnd_a: Vec<usize> = (&mut rng)
        .sample_iter(Uniform::new(10_000, 99_999))
        .take(MAX_ELEMENTS)
        .collect();
    let rnd_b: Vec<usize> = (&mut rng)
        .sample_iter(Uniform::new(10_000, 99_999))
        .take(MAX_ELEMENTS)
        .collect();

    for i in 1..MAX_ITERS {
        let n_items: usize = 10_usize.pow(i);
        println!(
            "\n---------------- Benchmarking {:} items -----------------",
            n_items
        );

        group.bench_with_input(BenchmarkId::new("Dict", n_items), &n_items, |b, n| {
            b.iter(|| day_01::part_2_dict(&rnd_a[0..*n], &rnd_b[0..*n]))
        });
        group.bench_with_input(BenchmarkId::new("Vec", n_items), &n_items, |b, n| {
            b.iter(|| day_01::part_2_vec(&rnd_a[0..*n], &rnd_b[0..*n]))
        });
    }
}

pub fn day_02(c: &mut Criterion) {
    //Result
    let list = day_02::loader();
    println!("Day 02 | Part 1: {:?}", day_02::part_1(&list));
    println!("Day 02 | Part 2: {:?}", day_02::part_2(&list));
    println!("Day 02 | Part 2: {:?}", day_02::part_2_opt(&list));

    //Part 1
    c.bench_function("Day_02/Loader", |b| b.iter(|| day_02::loader()));
    c.bench_function("Day_02/Part_1", |b| b.iter(|| day_02::part_1(&list)));
    let mut group = c.benchmark_group("Day_02/Part_2");
    group.bench_function("Baseline", |b| b.iter(|| day_02::part_2(&list)));
    group.bench_function("Optimized", |b| b.iter(|| day_02::part_2_opt(&list)));
}

criterion_group!(advent_2024, day_01, day_02);

criterion_main!(advent_2024);
