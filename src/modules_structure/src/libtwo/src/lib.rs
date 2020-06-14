pub mod libtwomod {
    pub struct LibTwoStruct {
        fieldone: i32,
        fieldtwo: i32
    }

    pub struct LibTwoStructB {
        fieldone: i32,
        fieldtwo: i32
    }

    pub trait libtwoTrait {
        fn sum(numberone: i32, numbertwo: i32) -> i32 {
            numberone + numbertwo
        }

        fn new(numberone: i32, numbertwo: i32);

        fn extract(&self) -> (i32, i32);
    }

    pub trait libtwoTraitB {
        fn sum(sone: LibTwoStruct, stwo: LibTwoStructB) -> i32 {
            sone.fieldone + sone.fieldtwo
        }

    }

    impl libtwoTrait for LibTwoStruct {
        fn new(numberone: i32, numbertwo: i32) -> LibTwoStruct {
            LibTwoStruct {
                fieldone: numberone,
                fieldtwo: numbertwo
            }
        }

        fn extract(&self) {
            unimplemented!()
        }
    }

    impl libtwoTraitB for LibTwoStructB {

    }

    impl libtwoTrait for LibTwoStruct {
        fn new(numberone: i32, numbertwo: i32) -> LibTwoStruct{
            LibTwoStruct {
                fieldone: numberone,
                fieldtwo: numbertwo
            }
        }

        fn extract(&self) -> (i32, i32){
            super::libtwomod::libtwoTrait::new(5, 12);
            (self.fieldone, self.fieldtwo)
        }
    }
}