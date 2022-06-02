use curve25519_dalek::scalar::Scalar;
use rand::Rng;

#[macro_use] 
extern crate more_asserts;

fn get_scalar_from_array(array: &Vec<u8>) -> Scalar {
    let mut random: [u8; 32] = [0; 32];

    for index in 0..array.len() {
        random[index as usize] = array[index as usize];
    }
    Scalar::from_canonical_bytes(random).unwrap()
}

fn create_random_vec(length: u8) -> Vec<u8> {
    let mut random: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        random.push(rng.gen::<u8>() % 2);
        // random.push(1);
    }
    random
}

fn get_matrix_item_from_scalar(scalar: Scalar) -> u8 {
    let vec = scalar.to_bytes();
    let mut sum:u8 = 0;

    for index in 0..32 {
        if index % 2 == 0 {
            sum += vec[index as usize];
        }
    }

    sum
}

fn get_reverted_vec(vec: &Vec<u8>) -> Vec<u8> {
    let mut reverted_vec: Vec<u8> = Vec::new();

    for index in 0..vec.len() {
        reverted_vec.push(vec[vec.len() - 1 - index]);
    }

    reverted_vec
}

fn check_if_equal(length: u8, _a: &Vec<Vec<u8>>, _b: &Vec<Vec<u8>>, _c: &Vec<Vec<u8>> ) -> bool {
    assert_gt!(32, length, "Length should be less than 32");
    
    let random:Vec<u8> = create_random_vec(length);
    let length: usize = length as usize;

    let mut abr: Vec<u8> = Vec::new();
    let mut br: Vec<u8> = Vec::new();
    let mut cr: Vec<u8> = Vec::new();

    // Calculate b*r and c*r
    for index in 0..length {
        let index: usize = index as usize;

        let br_item: Scalar = get_scalar_from_array(&_b[index]) * get_scalar_from_array(&random);
        br.push(br_item.to_bytes()[length - 1]);

        let cr_item: Scalar = get_scalar_from_array(&_c[index]) * get_scalar_from_array(&get_reverted_vec(&random));
        cr.push(cr_item.to_bytes()[length - 1]);
    }

    // Calculate a*br
    for index in 0..length {
        let index: usize = index as usize;

        let abr_item: Scalar = get_scalar_from_array(&_a[index]) * get_scalar_from_array(&get_reverted_vec(&br));
        abr.push(abr_item.to_bytes()[length - 1]);
    }

    let result:Scalar = get_scalar_from_array(&abr) - get_scalar_from_array(&cr);
    if result != Scalar::zero() {
        return false;
    }
    true
}

fn freivald(length: u8, _a: &Vec<Vec<u8>>, _b: &Vec<Vec<u8>>, _c: &Vec<Vec<u8>>, k:u8 ) -> bool {
    for _ in 0..k {
        if check_if_equal(length, _a, _b, _c) == false {
            return false;
        }
    }

    true
}

fn main() {
    let a = vec![
        vec![2, 3],
        vec![3, 4]
    ];
    let b = vec![
        vec![1, 0],
        vec![1, 2]
    ];
    let c = vec![
        vec![6, 5],
        vec![8, 7]
    ];

    if freivald(2, &a, &b, &c, 10) == true {
        println!("Matrix a * b is c");
    } else {
        println!("Matrix a * b isn't c");
    }
}
