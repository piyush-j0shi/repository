use adder::add_two;

#[test]
fn it_adds_two() {
    let result = add_two(3);
    assert_eq!(result, 5)
}

// to run a particular integration test
// cargo test --test integration_test
