

pub mod subject{
    #[derive(Debug)]
    pub struct Subject{
        pub name: String,
        pub arguments: Vec<Argument>
    }
    #[derive(Debug)]
    pub struct Argument{
        pub name: String,
    }
}
