use std::mem;

#[derive(PartialEq)]
#[derive(Debug)]
enum IpAddr {
    V4,
    V6,
}

struct Singleton {
    field: Option<IpAddr>
}

impl Singleton {
    fn take_instance(&mut self) -> IpAddr {
        let p = mem::replace(&mut self.field, None);
        p.unwrap()
    }
}

static mut SINGLETON: Singleton = Singleton {
    field: Some(IpAddr::V4),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let singleton_num = unsafe { SINGLETON.take_instance() };
        assert_eq!(singleton_num, IpAddr::V4);
    }

    #[test]
    #[should_panic]
    fn request_taken_singleton() {
        let singleton_num = unsafe { SINGLETON.take_instance() };
        assert_eq!(singleton_num, IpAddr::V4);
        let singleton_num2 = unsafe { SINGLETON.take_instance() };
    }

}
