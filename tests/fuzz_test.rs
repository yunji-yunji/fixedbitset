use fixedbitset::FixedBitSet;
use std::error::Error;

fn run(a_len: usize, a_start: usize, a_end: usize, b_len: usize, b_start: usize, b_end: usize) -> Result<(), Box<dyn Error>> {
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
/*
    // 1. symmetric_difference
    a.set_range(a_start..a_end, true);
    b.set_range(b_start..b_len, true);
    let count = a.symmetric_difference_count(&b);
    let iterator_count = a.symmetric_difference(&b).count();
    let a_sym_diff_b = a.symmetric_difference(&b).collect::<FixedBitSet>();
    for i in 0..a_start {
        assert!(!a_sym_diff_b.contains(i));
    }
    for i in a_start..b_start {
        assert!(a_sym_diff_b.contains(i));
    }
    for i in b_start..a_end {
        assert!(!a_sym_diff_b.contains(i));
    }
    for i in a_end..b_len {
        assert!(a_sym_diff_b.contains(i));
    }

    a.symmetric_difference_with(&b);
    assert_eq!(
        a_sym_diff_b, a,
        "symmetric_difference and _with produce the same results"
    );
    assert_eq!(
        a_sym_diff_b.count_ones(..),
        count,
        "symmetric_difference and _count produce the same results"
    );
    assert_eq!(
        count, iterator_count,
        "symmetric_difference and _count produce the same results"
    );

    // 2. bitor_first_smaller
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(..a_end, true);
    b.set_range(b_start.., true);
    let ab = &a | &b;
    for i in 0..a_end {
        assert!(ab.contains(i));
    }
    for i in a_end..b_start {
        assert!(!ab.contains(i));
    }
    for i in b_start..b_len {
        assert!(ab.contains(i));
    }
    assert_eq!(b_len, ab.len());

    // 3. bitor_first_larger
    let mut a = FixedBitSet::with_capacity(a_len);
    let mut b = FixedBitSet::with_capacity(b_len);
    a.set_range(a_start.., true);
    b.set_range(..b_end, true);
    let ab = &a | &b;
    for i in a_start..a_len {
        assert!(ab.contains(i));
    }
    for i in 0..b_end {
        assert!(ab.contains(i));
    }
    for i in b_end..a_start {
        assert!(!ab.contains(i));
    }
    assert_eq!(a_len, ab.len());

    let iters = if cfg!(miri) { 48 } else { 128 };
    for i in 1..iters {
        let tmp = FixedBitSet::with_capacity(i);
        for j in 0..tmp.len() + 1 {
            for k in j..tmp.len() + 1 {
                assert_eq!(tmp.count_ones(j..k), 0);
            }
        }
    }
*/
    // 4. from_iterator_ones
    for i in (0..a_len).filter(|i| i % 7 == 0) {
        a.put(i);
    }
    a.put(a_len - 1);
    let dup = a.ones().collect::<FixedBitSet>();

    assert_eq!(a.len(), dup.len());
    assert_eq!(
        a.ones().collect::<Vec<usize>>(),
        dup.ones().collect::<Vec<usize>>()
    );

    // 6. test_is_full
    a.grow(1);
    a.put(0);
    a.grow(42);
    a.clear();
    let _ = a.is_full();
    a.insert_range(..);
    assert!(a.is_full());

    // 7. count_ones
    let mut a = FixedBitSet::with_capacity(a_end);
    for i in a_start..a_end {
        a.insert(i);
    }
    let ones: Vec<_> = a.ones().collect();
    let expected: Vec<_> = (a_start..a_end).collect();
    let ones_rev: Vec<_> = a.ones().rev().collect();
    let expected_rev: Vec<_> = (a_start..a_end).rev().collect();
    assert_eq!(expected, ones);
    assert_eq!(expected_rev, ones_rev);

    a.count_ones(a_start..a_end);
    b.count_ones(b_start..b_end);

    // 5. op_assign_ref
    a &= &b;
    a |= &b;
    a ^= &b;

    b &= &a;
    b |= &a;
    b ^= &a;

    Ok(())
}

#[test]
fn my_fuzz() {
    let args: Vec<String> = std::env::args().collect();
    let mut data_arg: Option<String> = None;
    for arg in args.iter().skip(1) {
        if arg.starts_with("data=") {
            data_arg = Some(arg.chars().skip(5).collect());
            break;
        }
    }
    if let Some(data) = data_arg {
        let data_vec: Vec<usize> = data.split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect();

        let (a_len, a_start, a_end, b_len, b_start, b_end) = (
            data_vec[0],
            data_vec[1],
            data_vec[2],
            data_vec[3],
            data_vec[4],
            data_vec[5],
        );
        println!("\n- input data: {:?}", data);
        let res = run(a_len, a_start, a_end, b_len, b_start, b_end);
        println!("- result: {:?}", res);
    } else {
        panic!("input data not found");
    }
}

#[test]
fn quick_test() {
    let a_len = 137;
    let a_start = 32;
    let a_end = 120;
    let b_len = 173;
    let b_start = 50;
    let b_end = 107;
    let res = run(a_len, a_start, a_end, b_len, b_start, b_end);
    println!("- result: {:?}", res);
}