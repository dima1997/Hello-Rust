#[cfg(test)]
mod tests {
    struct MockStruct {
        option_b1: Option<Box<i64>>
    }

    #[test]
    fn it_should_be_moved_out_of_struct() {
        let mut ms = MockStruct {
            option_b1: Some(Box::new(1))
        };

        let option_b2 = ms.option_b1.take();
        
        if let Some(b2) = option_b2 {
            assert_eq!(*b2, 1);
        } else {
            assert!(false);
        }
        
        assert_eq!(ms.option_b1, None);
    }
}