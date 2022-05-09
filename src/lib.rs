use rstest_reuse;

#[macro_use]
pub mod template {
    use rstest_reuse::template;

    // Import the function to be used here
    use crate::values::the_number_two;

    // This doesn't work `the_number_two() not found in scope`.
    #[template]
    #[rstest(a,  b, case(the_number_two(), 2), case(4/2, 2))]
    fn template_that_does_not_work(a: u32, b: u32) {}

    // This template imports with the full module path, it works.
    #[template]
    #[rstest(a,  b, case(crate::values::the_number_two(), 2), case(4/2, 2))]
    fn template_that_works(a: u32, b: u32) {}
}

pub mod values {
    pub fn the_number_two() -> u32 {
        2
    }
}

#[cfg(test)]
pub mod test {
    use rstest::rstest;
    use rstest_reuse::apply;

    #[apply(template_that_does_not_work)]
    fn it_does_not_works(a: u32, b: u32) {
        assert!(a == b);
    }

    #[apply(template_that_works)]
    fn it_works(a: u32, b: u32) {
        assert!(a == b);
    }
}
