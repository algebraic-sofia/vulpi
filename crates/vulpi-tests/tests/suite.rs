#![feature(custom_test_frameworks)]
#![test_runner(vulpi_tests::test_runner)]

use vulpi_tests::test;

test!("/suite", |file_name| {
    todo!()
});
