use std::io;
use inline_colorization::*;
use rustproject::lib::{self, Budget};
use std::fs;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
struct AppState {
    user: lib::User,
    // Add any other data you want to save here
}
fn main() {
    let mut user = lib::User {
        transactions: vec![],
        budgets: vec![],
    };
    // let mut app_state = load_state().unwrap_or_else(|_| AppState {
    //     user: lib::User {
    //         transactions: vec![],
    //         budgets: vec![],
    //     },
    // });

    loop {
        println!("{color_blue}**************************************************************************************************************{color_reset}");
        println!("{style_underline}{style_bold}Welcome to the Income and Expense Management System{style_reset}");
        println!("");
        println!("{color_green}1. Add Income{color_reset}");
        println!("");
        println!("{color_red}2. Add Expense{color_reset}");
        println!("");
        println!("{color_yellow}3. View Transactions{color_reset}");
        println!("");
        println!("{color_blue}4. Set Budget{color_reset}");
        println!("");
        println!("{style_bold}5. Evaluate total{style_reset}");
        println!("");
        println!("{color_black}{style_underline}6. Exit{color_reset}{style_reset}");
        println!("");
        println!("{color_blue}***************************************************************************************************************{color_reset}");

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => {
                user.transactions.push(add_transaction("income"));
            }
            "2" => {
                user.transactions.push(add_expense("expense"));
            }
            "3" => {
                view_transactions(&user.transactions, &user.budgets, &user);
            }
            "4" => {
                user.budgets = set_and_display_budgets(&user.budgets);
            }
            "5" => {
                evaluate_total(&user.transactions);
            }
            "6" => {
                // save_state(&app_state);
                println!("Goodbye!");
                break;

            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn add_transaction(transaction_type: &str) -> lib::Transaction {
    let name = get_user_input("Enter transaction name: ");
    let amount: f64;
    loop {
        let amount_input = get_user_input("Enter transaction amount: ");
        match amount_input.trim().parse() {
            Ok(parsed_amount) => {
                amount = parsed_amount;
                break;
            }
            Err(_) => {
                println!("Invalid amount. Please enter a valid number.");
            }
        }
    }
    let date = get_user_input("Enter transaction date: ");
    let category = get_user_input("Enter transaction category: ");
    let is_income = true;

    lib::Transaction {
        name,
        amount,
        date,
        category,
        is_income
    }
}

fn add_expense(transaction_type: &str) -> lib::Transaction {
    let name = get_user_input("Enter transaction name: ");
    let mut amount: f64;

    loop {
        let amount_input = get_user_input("Enter transaction amount: ");
        match amount_input.trim().parse::<f64>() {
            Ok(parsed_amount) => {
                if parsed_amount > 0.0 {
                    amount = -parsed_amount;
                } else {
                    amount = parsed_amount;
                }
                break;
            }
            Err(_) => {
                println!("Invalid amount. Please enter a valid number.");
            }
        }
    }

    let date = get_user_input("Enter transaction date: ");
    let category = get_user_input("Enter transaction category: ");
    let is_income = false;
    lib::Transaction {
        name,
        amount,
        date,
        category,
        is_income
    }
}

fn view_transactions(transactions: &Vec<lib::Transaction>, budgets: &Vec<lib::Budget>, user: &lib::User) {
    fn print_transaction(transaction: &lib::Transaction, is_income : bool) {
        if is_income{
            println!("{color_reset}================================================================================================");
            println!("Name: {}", transaction.name);
            println!("Amount: {color_green}{}{color_reset}", transaction.amount);
            println!("Date: {}", transaction.date);
            println!("Category: {}", transaction.category);
            println!("{color_reset}================================================================================================");
            return
        }else{
            println!("{color_reset}================================================================================================");
            println!("Name: {}", transaction.name);
            println!("Amount: {color_red}{}{color_reset}", transaction.amount);
            println!("Date: {}", transaction.date);
            println!("Category: {}", transaction.category);
            println!("{color_reset}================================================================================================");
            return
        }
    }
    fn view_income_transactions(transactions: &Vec<lib::Transaction>) {
        println!("{color_reset}================================================================================================");
        println!("{style_underline}{color_green}Income Transactions{color_reset}{style_reset}");
        for transaction in transactions.iter().filter(|t| t.is_income) {
            print_transaction(transaction, true);
        }
    }
    fn view_expense_transactions(transactions: &Vec<lib::Transaction>, budgets: &Vec<lib::Budget>) {
        println!("{color_reset}================================================================================================");
        println!("{style_underline}{color_red}Expense Transactions{color_reset}{style_reset}");
    
        // Create a HashMap to accumulate expenses by category
        let mut category_expenses: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    
        // Accumulate expenses for each category
        for transaction in transactions.iter().filter(|t| !t.is_income) {
            let transaction_category = transaction.category.trim().to_lowercase();
            let expense = transaction.amount;
    
            // Update or insert the category's total expense
            let total_expense = category_expenses.entry(transaction_category.clone()).or_insert(0.0);
            *total_expense += expense;
    
            print_transaction(transaction, false);
        }
    
      
        // Display budget information for each category
        println!("{color_reset}================================================================================================");
        println!("{style_underline}{style_bold}Budget Summary{style_reset}");
        for budget in budgets.iter() {
            let total_expense: f64 = transactions
                .iter()
                .filter(|t| !t.is_income && t.category.trim().to_lowercase() == budget.category.trim().to_lowercase())
                .map(|t| t.amount)
                .sum();
    
            println!("{style_bold}Category: {color_blue}{}{color_reset}", budget.category);
            println!("Budget Amount: {color_blue}{}{color_reset}", budget.amount);
            println!("Total Expenses: {color_red}{}{color_reset}", total_expense);
    
            let remaining_budget = budget.amount + total_expense;
            if remaining_budget >= 0.0{
            println!("{style_reset}{color_green}Budget Remaining: {}{color_reset}", remaining_budget);
            }
            else{
                println!("{style_reset}{color_red}Budget Exceeded: {}{color_reset}", remaining_budget);
            }
        }
            println!("{color_reset}================================================================================================");
        }
    

    fn view_both_transactions(transactions: &Vec<lib::Transaction>, budgets: &Vec<lib::Budget>, user: &lib::User) {
        println!("{color_reset}================================================================================================");
        println!("{style_underline}{style_bold}All Transactions{style_reset}");
    
        for transaction in transactions.iter() {
            if transaction.is_income {
                print_transaction(transaction, true);
            } else {
                print_transaction(transaction, false);
        }
    }
    
        println!("{color_reset}================================================================================================");
        println!("{style_underline}{style_bold}Budget Summary{style_reset}");
        println!("");
        for budget in budgets.iter() {
            let total_expense: f64 = transactions
                .iter()
                .filter(|t| !t.is_income && t.category.trim().to_lowercase() == budget.category.trim().to_lowercase())
                .map(|t| t.amount)
                .sum();
    
            println!("{style_bold}Category: {color_blue}{}{color_reset}", budget.category);
            println!("Budget Amount: {color_blue}{}{color_reset}", budget.amount);
            println!("Total Expenses: {color_red}{}{color_reset}", total_expense);
    
            let remaining_budget = budget.amount + total_expense;
            if remaining_budget >= 0.0{
                println!("{style_reset}{color_green}Budget Remaining: {}{color_reset}", remaining_budget);
                println!("");
            }
            else{
                println!("{style_reset}{color_red}Budget Exceeded: {}{color_reset}", remaining_budget);
                println!("");
            }
            
            println!("{color_reset}================================================================================================");
        }
    }


    if transactions.is_empty() {
        println!("No transactions to display.");
        return;
    }

    println!("Do you want to view income, expense, or both transactions? ({color_green}I{color_reset} for {color_green}Income{color_reset}, {color_red}E{color_reset} for {color_red}Expense{color_reset}, {style_bold}B{style_reset} for {style_bold}Both{style_reset}): ");
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect("Failed to read input");
    let user_choice = user_choice.trim().to_ascii_lowercase();
    let budgets = &user.budgets;

    match user_choice.as_str() {
        "i" => view_income_transactions(transactions),
        "e" => view_expense_transactions(transactions, budgets),
        "b" => view_both_transactions(transactions, budgets, user),
        _ => println!("Invalid choice. Please enter 'I' for income, 'E' for expense, or 'B' for both."),
    }
}


fn set_and_display_budgets(budgets: &Vec<lib::Budget>) -> Vec<lib::Budget> {
    let mut updated_budgets = budgets.clone(); // Make a copy of the existing budgets

    loop {
        let choice = get_user_input("Enter budget category or type 'done' to finish: ");

        if choice.trim().to_lowercase() == "done" {
            break;
        }

        let category = choice.trim().to_string();
        let amount = get_user_input("Enter budget amount: ").trim().parse().expect("Invalid amount");

        let budget = lib::Budget { category, amount };
        updated_budgets.push(budget);

        println!("{color_blue}***************************************************************************************************************{color_reset}");
        println!("{style_underline}{style_bold}Currently Set Budgets{style_reset}");

        for (index, budget) in updated_budgets.iter().enumerate() {
            println!("");
            println!("{style_bold}Budget #{}{style_reset}", index + 1);
            println!("Category: {}", budget.category);
            println!("Amount: {}", budget.amount);
        }

        println!("{color_blue}***************************************************************************************************************{color_reset}");
    }

    updated_budgets.to_vec() // Convert to owned Vec
}

fn evaluate_total(transactions: &Vec<lib::Transaction>) {
    let total_income: f64 = transactions.iter().filter(|t| t.is_income).map(|t| t.amount).sum();
    let total_expense: f64 = transactions.iter().filter(|t| !t.is_income).map(|t| t.amount).sum();
    let net_money = total_income + total_expense;
    println!("{color_blue}***************************************************************************************************************{color_reset}");
    println!("{style_underline}{style_bold}Total Summary{style_reset}");
    println!("{color_green}Total Income: {}{color_reset}", total_income);
    println!("{color_red}Total Expense: {}{color_reset}", total_expense);
    if net_money > 0.0{
        println!("{color_green}Net Money: {}{color_reset}", net_money);
    }
    else if net_money == 0.0 {
        println!("{color_black}Net Money: {}{color_reset}", net_money);
    }
    else {
        println!("{color_red}Net Money: {}{color_reset}", net_money);
    }
    
}

// fn save_state(app_state: &AppState) {
//     let serialized_state = serde_json::to_string(app_state).expect("Serialization failed");
//     fs::write("state.json", serialized_state).expect("Failed to save state to disk");
// }

// fn load_state() -> Result<AppState, Box<dyn std::error::Error>> {
//     if let Ok(serialized_state) = fs::read_to_string("state.json") {
//         let app_state = serde_json::from_str(&serialized_state)?;
//         Ok(app_state)
//     } else {
//         Ok(AppState {
//             user: lib::User {
//                 transactions: vec![],
//                 budgets: vec![],
//             },
//         })
//     }
// }