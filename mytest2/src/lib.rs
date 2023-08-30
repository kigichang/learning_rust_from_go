pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_deref() {
        use std::ops::Deref;

        struct MyResult<T, E> {
            inner: Result<T, E>,
        }

        impl<T, E> Deref for MyResult<T, E> {
            type Target = Result<T, E>;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<T, E> From<Result<T, E>> for MyResult<T, E> {
            fn from(inner: Result<T, E>) -> Self {
                Self { inner }
            }
        }
    }

    /// .
    #[test]
    fn test_a() {
        println!("{:?}", &8_u32.to_le_bytes());

        let a = {
            let mut a = 1;
            a += 1;
            if a > 0 {
                return;
            }
            a
        };

        println!("{}", a);
    }

    #[test]
    fn test_lifetime() {
        let r;
        {
            let x = 5;
            r = &x;
            println!("r: {}", r);
        }
        //println!("r: {}", r);
    }

    #[test]
    fn test_fn_lifetime() {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let x = "abcd";
        let y = "abc";
        println!("{}", longest(x, y));

        let string1 = String::from("abcd");
        let result;

        {
            let string2 = String::from("xyz");
            result = longest(&string1, &string2);
            println!("The longest string is {}", result);
        }
        //println!("The longest string is {}", result);
    }

    #[test]
    fn test_method_lifetime() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }

            fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'b str
            where
                'a: 'b,
            {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
    }

    #[test]
    fn test_static_lifetime() {
        fn print_author(author: &'static str) {
            println!("{}", author);
        }

        {
            let x = "abcd";
            print_author(x);
        }

        fn static_bound<T: std::fmt::Display + 'static>(t: T) {
            println!("{}", t);
        }

        fn static_bound_ref<T: std::fmt::Display + 'static>(t: &T) {
            println!("{}", t);
        }

        {
            let x = "abcd";
            static_bound(x);
            let i = 32;
            static_bound(i);
            //static_bound(&i);
            static_bound_ref(&i);
        }
    }

    #[test]
    fn test_display() {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Serialize, Deserialize)]
        struct MyTest {
            id: u32,
            name: String,
        }

        impl Display for MyTest {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                //write!(f, "id: {}, name: {}", self.id, self.name)
                //f.write_fmt(format_args!("id: {}, name: {}", self.id, self.name))
                f.write_str(&serde_json::to_string(self).unwrap())
            }
        }

        let t = MyTest {
            id: 1,
            name: "abcd".to_string(),
        };

        println!("{t}");
        println!("{t:?}");
    }

    #[test]
    fn test_struct_borrow() {
        #[derive(Debug, Clone)]
        struct Test {
            name: String,
            age: i32,
        }

        fn aa(src: String) {
            println!("{}", src)
        }

        let t = Test {
            name: "abcd".to_string(),
            age: 10,
        };

        aa(t.name);

        let t = t.clone();

        println!("{:?}", t);
    }
}
