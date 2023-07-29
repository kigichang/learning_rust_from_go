fn main() {
    //{
    //    trait MyTrait {
    //        fn hello(&self) -> String;
    //    }
//
    //    struct My1;
    //    struct My2;
//
    //    impl MyTrait for My1 {
    //        fn hello(&self) -> String {
    //            "My1".to_string()
    //        }
    //    }
//
    //    impl Drop for My1 {
    //        fn drop(&mut self) {
    //            println!("My1 drop");
    //        }
    //    }
//
    //    impl MyTrait for My2 {
    //        fn hello(&self) -> String {
    //            "My2".to_string()
    //        }
    //    }
    //    impl Drop for My2 {
    //        fn drop(&mut self) {
    //            println!("My2 drop");
    //        }
    //    }
//
    //    fn test_trait1(_: bool) -> impl MyTrait {
    //        My1
    //    }
//
    //    fn test_trait2(a: bool) -> Box<impl MyTrait> {
    //        if a {
    //            Box::new(My2)
    //        } else {
    //            Box::new(My2)
    //        }
    //    }
//
    //    fn test_trait3(a: bool) -> Box<dyn MyTrait> {
    //        if a {
    //            Box::new(My1)
    //        } else {
    //            Box::new(My2)
    //        }
    //    }
//
    //    fn test_trait4(a: impl MyTrait) {
    //        println!("test_trait4:{:?}", a.hello());
    //    }
//
    //    //fn test_trait5(a: dyn MyTrait) {
    //    //    println!("test_trait5:{:?}", a.hello());
    //    //}
//
    //    fn test_trait_generic<T: MyTrait>(a: T) {
    //        println!("test_trait_generic:{:?}", a.hello());
    //    }
//
    //    println!("test_trait1:{:?}", test_trait1(true).hello());
    //    println!("test_trait1:{:?}", test_trait1(false).hello());
//
    //    println!("test_trait2:{:?}", test_trait2(true).hello());
    //    println!("test_trait2:{:?}", test_trait2(false).hello());
//
    //    println!("test_trait3:{:?}", test_trait3(true).hello());
    //    println!("test_trait3:{:?}", test_trait3(false).hello());
//
    //    test_trait4(My1);
    //    test_trait4(My2);
//
    //    let mut smart_ptr: Box<dyn MyTrait> = Box::new(My1);
    //    println!("smart_ptr:{:?}", smart_ptr.hello());
    //    smart_ptr = Box::new(My2);
    //    println!("smart_ptr:{:?}", smart_ptr.hello());
//
    //    test_trait_generic(My1);
    //    test_trait_generic(My2);
    //}



    {

        struct My1<'a, 'b> {
            pub id: &'a str,
            pub name: &'b str,

        }


        impl<'a, 'b> My1<'a, 'b> {
            fn new(id: &'a str, name: &'b str) -> Self {
                My1 { id, name }
            }

            fn hello(&self) {
                println!("{} hello:{}", self.id, self.name);
            }
        }
        unsafe impl<'a ,'b> Send for My1<'a, 'b>{}
        unsafe impl<'a, 'b> Sync for My1<'a, 'b>{}

        let id1 = "id1".to_string();
        {
            let name1 = "name1".to_string();

            {
                let id1 = &id1;
                let name1 = &name1;

                std::thread::spawn(move || {
                    let my1 = My1::new(&id1, &name1);
                    my1.hello();
                });
            }
        }
        
    }
}
