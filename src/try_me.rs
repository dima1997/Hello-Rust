#[cfg(test)]
mod tests {
    use std::{sync::{Arc, RwLock}, thread};

    fn wrap_f2(l_count: &Arc<RwLock<i64>>) -> Box<dyn Fn() -> () + Send> {
        let c_l_count = l_count.clone();
        Box::new(move || {
            if let Ok(mut count) = c_l_count.write() {
                *count += 1;
            };
        })
    }

    fn f1(f2: Box<dyn Fn() -> () + Send>) {
        let handler = thread::spawn(move || f2());
        handler.join().unwrap();
    }

    #[test]
    fn some_test() {
        let l_count = Arc::new(RwLock::new(0));
        
        let f2 = wrap_f2(&l_count);
        f1(f2);

        let count = l_count.read().unwrap();
        assert_eq!(*count, 1);
    }
}