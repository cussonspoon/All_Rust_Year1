pub mod lib {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Clone)]
    pub struct Transaction {
        pub name: String,
        pub amount: f64,
        pub date: String,
        pub category: String,
        pub is_income: bool,
    }
    #[derive(Clone, Serialize, Deserialize)]
    pub struct Budget {
        pub category: String,
        pub amount: f64,
    }
    #[derive(Serialize, Deserialize)]
    pub struct User {
        pub transactions: Vec<Transaction>,
        pub budgets: Vec<Budget>,
    }
}
