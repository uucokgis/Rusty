mod basics {

    pub trait Draw {
        fn draw(&self);
    }

    pub struct Screen<T: Draw> {
        pub components: Vec<Box<dyn Draw>>,
        pub componentsv2 : Vec<T>
    }

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String
    }

    pub struct SelectBox{
        width: u32,
        height: u32,
        options: Vec<String>
    }

    impl Draw for SelectBox{
        fn draw(&self) {

        }
    }

    impl <T> Screen <T>
        where T: Draw

    {
        pub fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    impl Draw for Button {
        fn draw(&self) {
        }
    }

    pub fn creategui () {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No")
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
            componentsv2: vec![]
        };

        screen.run();
    }
}