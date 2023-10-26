
pub mod lib { 
    pub struct Transaction {
        pub name: String,
        pub amount: f64,
        pub date: String,
        pub category: String,
        pub is_income: bool,
    }
    #[derive(Clone)]
    pub struct Budget {
        pub category: String,
        pub amount: f64,
    }
    pub struct User {
        pub transactions: Vec<Transaction>,
        pub budgets: Vec<Budget>,
    }
}
