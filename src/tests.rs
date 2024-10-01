extern crate test;

#[cfg(test)]
mod tests {
    extern crate env_logger;

    use ::{do_nothing, setup_logging};

    #[test]
    fn test_do_nothing() {
        setup_logging();
        do_nothing();
    }
}