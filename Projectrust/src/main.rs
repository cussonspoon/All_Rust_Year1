use inline_colorization::*;
use rustproject::lib::Transaction;
use rustproject::lib::{self, Budget};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

#[derive(Serialize, Deserialize)]
struct AppState {
    user: lib::User,
}
fn main() {
    let mut user = lib::User {
        transactions: vec![],
        budgets: vec![],
    };
    let mut app_state = load_state().unwrap_or_else(|_| AppState {
        user: lib::User {
            transactions: vec![],
            budgets: vec![],
        },
    });
    if !app_state.user.transactions.is_empty() {
        user.transactions = app_state.user.transactions.clone();
    }
    if !app_state.user.budgets.is_empty() {
        user.budgets = app_state.user.budgets.clone();
    }

    loop {
        println!("{color_blue}**************************************************************************************************************{color_reset}");
        println!("{style_underline}{style_bold}Welcome to the Income and Expense Management System{style_reset}");
        println!("");
        println!("{color_green}1. Add Income{color_reset}");
        println!("");
        println!("{color_red}2. Add Expense{color_reset}");
        println!("");
        println!("{color_white}3. Edit Transactions{color_reset}");
        println!("");
        println!("{color_yellow}4. View Transactions{color_reset}");
        println!("");
        println!("{color_blue}5. Set or View Budget{color_reset}");
        println!("");
        println!("{style_bold}6. Evaluate total{style_reset}");
        println!("");
        println!("{color_cyan}7. Export as HTML{color_reset}");
        println!("");
        println!("{color_black}{style_underline}8. Save and Exit{color_reset}{style_reset}");
        println!("");
        println!("{color_blue}***************************************************************************************************************{color_reset}");

        let choice = get_user_input("Enter your choice: ");

        match choice.trim() {
            "1" => {
                user.transactions
                    .push(add_transaction(&user.transactions, "income"));
            }
            "2" => {
                user.transactions
                    .push(add_expense(&user.transactions, "expense"));
            }
            "3" => {
                let edit_type =
                    get_user_input("Do you want to edit a transaction or a budget? (t/b): ")
                        .trim()
                        .to_ascii_lowercase();
                edit_transactions(&mut user.transactions, &mut user.budgets, &edit_type);
            }

            "4" => {
                view_transactions(&user.transactions, &user.budgets, &user);
            }
            "5" => {
                user.budgets = set_and_display_budgets(&user.budgets);
            }
            "6" => {
                evaluate_total(&user.transactions, &user.budgets);
            }
            "7" => loop {
                println!("Do you want to export by day, month, or year\n{style_bold}(d/m/y){style_reset} or {style_bold}e{style_reset} to exit");
                let export_type = get_user_input("").trim().to_ascii_lowercase();
                if export_type == "e" {
                    break;
                }
                export_as_html(&user.transactions, &user.budgets, &export_type)
            },

            "8" => {
                app_state.user.transactions.clear();
                app_state.user.budgets.clear();
                app_state
                    .user
                    .transactions
                    .extend(user.transactions.iter().cloned());
                app_state.user.budgets.extend(user.budgets.iter().cloned());
                save_state(&app_state);
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
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn add_transaction(
    transactions: &Vec<lib::Transaction>,
    transaction_type: &str,
) -> lib::Transaction {
    let mut count = transactions.len() as u32;
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
    count += 1;

    lib::Transaction {
        count,
        name,
        amount,
        date,
        category,
        is_income,
    }
}

fn add_expense(transactions: &Vec<lib::Transaction>, transaction_type: &str) -> lib::Transaction {
    let mut count = transactions.len() as u32;
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
    count += 1;

    lib::Transaction {
        count,
        name,
        amount,
        date,
        category,
        is_income,
    }
}
fn print_transaction(transaction: &lib::Transaction, is_income: bool) {
    if is_income {
        println!("{color_reset}================================================================================================");
        println!(
            "{style_bold}Transaction number {}{style_reset}",
            transaction.count
        );
        println!("Name: {}", transaction.name);
        println!("Amount: {color_green}{}{color_reset}", transaction.amount);
        println!("Date: {}", transaction.date);
        println!("Category: {}", transaction.category);
        println!("{color_reset}================================================================================================");
        return;
    } else {
        println!("{color_reset}================================================================================================");
        println!(
            "{style_bold}Transaction number {}{style_reset}",
            transaction.count
        );
        println!("Name: {}", transaction.name);
        println!("Amount: {color_red}{}{color_reset}", transaction.amount);
        println!("Date: {}", transaction.date);
        println!("Category: {}", transaction.category);
        println!("{color_reset}================================================================================================");
        return;
    }
}

fn view_transactions(
    transactions: &Vec<lib::Transaction>,
    budgets: &Vec<lib::Budget>,
    user: &lib::User,
) {
    fn view_income_transactions(transactions: &Vec<lib::Transaction>) {
        println!("{color_reset}================================================================================================");
        println!("{style_underline}{color_green}Income Transactions{color_reset}{style_reset}");
        for transaction in transactions.iter().filter(|t| t.is_income) {
            print_transaction(transaction, true);
        }
        let mut category_incomes: std::collections::HashMap<String, f64> =
            std::collections::HashMap::new();

        for transaction in transactions.iter().filter(|t| t.is_income) {
            let transaction_category = transaction.category.trim().to_lowercase();
            let income = transaction.amount;
            let total_income = category_incomes
                .entry(transaction_category.clone())
                .or_insert(0.0);
            *total_income += income;
        }
        println!("{style_underline}{style_bold}Total Income by Category{style_reset}:");
        println!("");
        for (category, total_income) in &category_incomes {
            println!(
                "{style_bold}{}{color_reset}: {color_green}{}{color_reset}",
                category, total_income
            );
        }
    }

    fn view_expense_transactions(transactions: &Vec<lib::Transaction>, budgets: &Vec<lib::Budget>) {
        println!("{color_reset}================================================================================================");
        println!("{style_underline}{color_red}Expense Transactions{color_reset}{style_reset}");

        let mut category_expenses: std::collections::HashMap<String, f64> =
            std::collections::HashMap::new();

        for transaction in transactions.iter().filter(|t| !t.is_income) {
            let transaction_category = transaction.category.trim().to_lowercase();
            let expense = transaction.amount;

            let total_expense = category_expenses
                .entry(transaction_category.clone())
                .or_insert(0.0);
            *total_expense += expense;

            print_transaction(transaction, false);
        }

        println!("{color_reset}================================================================================================");
        println!("{style_underline}{style_bold}Budget Summary{style_reset}");
        println!("");
        for budget in budgets.iter() {
            let total_expense: f64 = transactions
                .iter()
                .filter(|t| {
                    !t.is_income
                        && t.category.trim().to_lowercase() == budget.category.trim().to_lowercase()
                })
                .map(|t| t.amount)
                .sum();

            println!(
                "{style_bold}Category: {color_blue}{}{color_reset}",
                budget.category
            );
            println!("Budget Amount: {color_blue}{}{color_reset}", budget.amount);
            println!(
                "Total Expenses: {style_reset}{color_red}{}{color_reset}",
                total_expense
            );

            let remaining_budget = budget.amount + total_expense;
            if remaining_budget >= 0.0 {
                println!(
                    "{style_reset}{color_green}Budget Remaining: {}{color_reset}",
                    remaining_budget
                );
            } else {
                println!(
                    "{style_reset}{color_red}Budget Exceeded: {}{color_reset}",
                    remaining_budget
                );
            }
            println!("");
        }
        println!("{color_reset}================================================================================================");
    }

    fn view_both_transactions(
        transactions: &Vec<lib::Transaction>,
        budgets: &Vec<lib::Budget>,
        user: &lib::User,
    ) {
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
        let mut category_incomes: std::collections::HashMap<String, f64> =
            std::collections::HashMap::new();

        for transaction in transactions.iter().filter(|t| t.is_income) {
            let transaction_category = transaction.category.trim().to_lowercase();
            let income = transaction.amount;
            let total_income = category_incomes
                .entry(transaction_category.clone())
                .or_insert(0.0);
            *total_income += income;
        }
        println!("{style_underline}{style_bold}Total Income by Category{style_reset}:");
        println!("");
        for (category, total_income) in &category_incomes {
            println!(
                "{style_bold}{}{color_reset}: {color_green}{}{color_reset}",
                category, total_income
            );
        }
        println!("");
        println!("{color_reset}================================================================================================");
        println!("{style_underline}{style_bold}Budget Summary{style_reset}");
        println!("");
        for budget in budgets.iter() {
            let total_expense: f64 = transactions
                .iter()
                .filter(|t| {
                    !t.is_income
                        && t.category.trim().to_lowercase() == budget.category.trim().to_lowercase()
                })
                .map(|t| t.amount)
                .sum();

            println!(
                "{style_bold}Category: {color_blue}{}{color_reset}",
                budget.category
            );
            println!("Budget Amount: {color_blue}{}{color_reset}", budget.amount);
            println!("Total Expenses: {color_red}{}{color_reset}", total_expense);

            let remaining_budget = budget.amount + total_expense;
            if remaining_budget >= 0.0 {
                println!(
                    "{style_reset}{color_green}Budget Remaining: {}{color_reset}",
                    remaining_budget
                );
                println!("");
            } else {
                println!(
                    "{style_reset}{color_red}Budget Exceeded: {}{color_reset}",
                    remaining_budget
                );
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
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read input");
    let user_choice = user_choice.trim().to_ascii_lowercase();
    let budgets = &user.budgets;

    match user_choice.as_str() {
        "i" => view_income_transactions(transactions),
        "e" => view_expense_transactions(transactions, budgets),
        "b" => view_both_transactions(transactions, budgets, user),
        _ => println!(
            "Invalid choice. Please enter 'I' for income, 'E' for expense, or 'B' for both."
        ),
    }
}

fn set_and_display_budgets(budgets: &Vec<lib::Budget>) -> Vec<lib::Budget> {
    let mut count = budgets.len() as u32 + 1;
    let mut updated_budgets = budgets.clone();
    for (index, budget) in updated_budgets.iter().enumerate() {
        println!("");
        println!("{style_bold}Budget #{}{style_reset}", budget.count);
        println!("Category: {}", budget.category);
        println!("Amount: {color_green}{}{color_reset}", budget.amount);
    }
    loop {
        let choice = get_user_input("Enter budget category or type 'done' to exit: ");

        if choice.trim().to_lowercase() == "done" {
            break;
        }

        let category = choice.trim().to_string();
        let amount = get_user_input("Enter budget amount: ")
            .trim()
            .parse()
            .expect("Invalid amount");

        let budget = lib::Budget {
            count,
            category,
            amount,
        };
        updated_budgets.push(budget);
        count += 1;

        println!("{color_blue}***************************************************************************************************************{color_reset}");
        println!("{style_underline}{style_bold}Currently Set Budgets{style_reset}");

        for (index, budget) in updated_budgets.iter().enumerate() {
            println!("");
            println!("{style_bold}Budget #{}{style_reset}", budget.count);
            println!("Category: {}", budget.category);
            println!("Amount: {color_green}{}{color_reset}", budget.amount);
        }

        println!("{color_blue}***************************************************************************************************************{color_reset}");
    }

    updated_budgets.to_vec()
}

fn evaluate_total(transactions: &Vec<lib::Transaction>, budgets: &Vec<lib::Budget>) {
    println!("{color_blue}***************************************************************************************************************{color_reset}");
    println!("");
    println!("{color_blue}=={color_reset} {style_underline}{style_bold}Total Summary{style_reset} {color_blue}=={color_reset}");
    println!("");
    let mut category_incomes: std::collections::HashMap<String, f64> =
        std::collections::HashMap::new();

    for transaction in transactions.iter().filter(|t| t.is_income) {
        let transaction_category = transaction.category.trim().to_lowercase();
        let income = transaction.amount;

        let total_income = category_incomes
            .entry(transaction_category.clone())
            .or_insert(0.0);
        *total_income += income;
    }
    println!("{style_underline}{style_bold}Total Income by Category{style_reset}:");
    println!("");
    for (category, total_income) in &category_incomes {
        println!(
            "{style_bold}{}{color_reset}: {color_green}{}{color_reset}",
            category, total_income
        );
    }
    println!("");
    println!("{style_bold}{style_underline}Budgets summary{style_reset}");
    for budget in budgets.iter() {
        let total_expense: f64 = transactions
            .iter()
            .filter(|t| {
                !t.is_income
                    && t.category.trim().to_lowercase() == budget.category.trim().to_lowercase()
            })
            .map(|t| t.amount)
            .sum();

        println!(
            "{style_bold}Category: {color_blue}{}{color_reset}",
            budget.category
        );
        println!("Budget Amount: {color_blue}{}{color_reset}", budget.amount);
        println!("Total Expenses: {color_red}{}{color_reset}", total_expense);

        let remaining_budget = budget.amount + total_expense;
        if remaining_budget >= 0.0 {
            println!(
                "{style_reset}{color_green}Budget Remaining: {}{color_reset}",
                remaining_budget
            );
            println!("");
        } else {
            println!(
                "{style_reset}{color_red}Budget Exceeded: {}{color_reset}",
                remaining_budget
            );
            println!("");
        }
    }
    let total_income: f64 = transactions
        .iter()
        .filter(|t| t.is_income)
        .map(|t| t.amount)
        .sum();
    let total_expense: f64 = transactions
        .iter()
        .filter(|t| !t.is_income)
        .map(|t| t.amount)
        .sum();
    let net_money = total_income + total_expense;
    println!(
        "{style_underline}{color_green}Total Income{style_reset}: {color_green}{}{color_reset}",
        total_income
    );
    println!(
        "{style_underline}{color_red}Total Expense{style_reset}: {color_red}{}{color_reset}",
        total_expense
    );
    if net_money > 0.0 {
        println!(
            "{style_underline}{color_green}Net Money{style_reset}: {color_green}{}{color_reset}",
            net_money
        );
    } else if net_money == 0.0 {
        println!(
            "{style_underline}{color_black}Net Money{style_reset}: {color_black}{}{color_reset}",
            net_money
        );
    } else {
        println!(
            "{style_underline}{color_red}Net Money{style_reset}:{color_red}{}{color_reset}",
            net_money
        );
    }
}

fn save_state(app_state: &AppState) {
    let serialized_state = serde_json::to_string(app_state).expect("Serialization failed");
    fs::write("state.json", serialized_state).expect("Failed to save state to disk");
}

fn load_state() -> Result<AppState, Box<dyn std::error::Error>> {
    match fs::read_to_string("state.json") {
        Ok(serialized_state) => match serde_json::from_str(&serialized_state) {
            Ok(app_state) => {
                println!("Loaded previous statement.");
                Ok(app_state)
            }
            Err(err) => {
                eprintln!("Failed to deserialize state: {}", err);
                Err(err.into())
            }
        },
        Err(err) => {
            eprintln!("Failed to read state file: {}", err);
            Ok(AppState {
                user: lib::User {
                    transactions: vec![],
                    budgets: vec![],
                },
            })
        }
    }
}

fn edit_transactions(
    transactions: &mut Vec<lib::Transaction>,
    budgets: &mut Vec<lib::Budget>,
    edit_type: &str,
) {
    match edit_type {
        "t" => {
            if transactions.is_empty() {
                println!("No transactions to edit.");
                return;
            }

            let index = loop {
                println!("Enter the number of the transaction you want to edit: ");
                println!("{color_reset}================================================================================================");
                println!("{style_underline}{style_bold}All Transactions{style_reset}");

                for transaction in transactions.iter() {
                    if transaction.is_income {
                        print_transaction(transaction, true);
                    } else {
                        print_transaction(transaction, false);
                    }
                }
                let index_input = get_user_input("Enter the transaction number you want to edit: ");

                if let Ok(index) = index_input.trim().parse::<usize>() {
                    if index <= transactions.len() && index > 0 {
                        break index;
                    } else {
                        println!("Invalid index. Please enter a valid index.");
                    }
                } else {
                    println!("Invalid input. Please enter a valid index.");
                }
            };

            let transaction = &mut transactions[index - 1];
            println!("{color_reset}================================================================================================");
            println!("{style_underline}{style_bold}Editing transaction:{style_reset}");
            println!("Name: {}", transaction.name);
            if transaction.amount >= 0.0 {
                println!("Amount: {color_green}{}{color_reset}", transaction.amount);
            } else {
                println!("Amount: {color_red}{}{color_reset}", transaction.amount);
            }
            println!("Date: {}", transaction.date);
            println!("Category: {}", transaction.category);
            println!("What do you want to edit? (name/amount/date/category)\n{style_bold}(n/a/d/c){style_reset}");
            let choice = get_user_input("").trim().to_ascii_lowercase();

            match choice.as_str() {
                "n" => {
                    let new_name = get_user_input("Enter the new name: ");
                    transaction.name = new_name.trim().to_string();
                }
                "a" => {
                    let new_amount = get_user_input("Enter the new amount: ")
                        .trim()
                        .parse::<f64>();
                    loop {
                        if let Ok(new_amount) = new_amount {
                            if transaction.is_income {
                                transaction.amount = new_amount.abs();
                                break;
                            } else {
                                transaction.amount = -(new_amount.abs());
                                break;
                            }
                        } else {
                            println!("Invalid amount. Transaction not updated.");
                        }
                    }
                }
                "d" => {
                    let new_date = get_user_input("Enter the new date: ");
                    transaction.date = new_date.trim().to_string();
                }
                "c" => {
                    let new_category = get_user_input("Enter the new category: ");
                    transaction.category = new_category.trim().to_string();
                }
                _ => {
                    println!("Invalid choice. Transaction not updated.");
                }
            }
        }
        "b" => {
            if budgets.is_empty() {
                println!("No budget to edit.");
                return;
            }
            for budget in budgets.iter() {
                println!("");
                println!("{style_bold}Budget #{}{style_reset}", budget.count);
                println!("Category: {}", budget.category);
                println!("Amount: {color_green}{}{color_reset}", budget.amount);
            }

            println!("Enter the number of the budget category you want to edit: ");
            let index_input = get_user_input("Budget number: ").trim().parse::<usize>();

            if let Ok(index) = index_input {
                if index <= budgets.len() && index > 0 {
                    let index = index - 1;
                    let budget = &mut budgets[index];

                    println!("{color_reset}================================================================================================");
                    println!("{style_underline}{style_bold}Editing Budget Name{style_reset}:{color_blue}{}{color_reset}", budget.category);
                    println!("Amount: {color_green}{}{color_reset}", budget.amount);
                    println!(
                        "What do you want to edit? (name/amount)\n{style_bold}(n/a){style_reset}: "
                    );
                    let choice = get_user_input("").trim().to_ascii_lowercase();

                    match choice.as_str() {
                        "n" => {
                            let new_name = get_user_input("Enter the new name: ");
                            budget.category = new_name.trim().to_string();
                        }
                        "a" => {
                            let new_amount_input = get_user_input("Enter the new amount: ")
                                .trim()
                                .parse::<f64>();
                            if let Ok(new_amount) = new_amount_input {
                                budget.amount = new_amount;
                            } else {
                                println!("Invalid amount. Budget not updated.");
                            }
                        }
                        _ => {
                            println!("Invalid choice. Budget not updated.");
                        }
                    }
                } else {
                    println!("Invalid index. Please enter a valid index.");
                }
            } else {
                println!("Invalid input. Please enter a valid index.");
            }
        }
        _ => {
            println!("Invalid edit type. Please specify 'transaction(t)' or 'budget(b)'.");
        }
    }
}

fn export_as_html(
    transactions: &Vec<lib::Transaction>,
    budgets: &Vec<lib::Budget>,
    export_type: &str,
) {
    let mut filename = String::default();
    let mut export_data = String::new();
    let mut total_income = 0.0;
    let mut total_expense = 0.0;
    match export_type.trim() {
        "d" => {
            let unique_dates: Vec<String> = transactions
                .iter()
                .map(|transaction| transaction.date.trim().to_string())
                .collect();

            let mut unique_dates = unique_dates
                .into_iter()
                .collect::<std::collections::HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();
            unique_dates.sort();

            println!("Available dates to choose from:");
            for (index, date) in unique_dates.iter().enumerate() {
                println!("{}: {}", index + 1, date);
            }

            println!("Enter the index of the date you want to export: ");
            let date_index = get_user_input("").trim().parse::<usize>();

            match date_index {
                Ok(index) => {
                    if index > 0 && index <= unique_dates.len() {
                        let selected_date = &unique_dates[index - 1];
                        filename = format!("export_{}.html", selected_date.replace("/", "_"));
                        export_data
                            .push_str("<html>\n<head>\n<title>Income and Expense Report</title>\n");
                        export_data.push_str("<style>\n");
                        export_data
                            .push_str("table {\nborder-collapse: collapse;\nwidth: 100%;\n}\n");
                        export_data.push_str("table, th, td {\nborder: 1px solid black;\n}\n");
                        export_data.push_str("th, td {\npadding: 10px;\ntext-align: left;\n}\n");
                        export_data.push_str("th {\nbackground-color: #333;\ncolor: white;\n}\n");
                        export_data
                            .push_str("tr:nth-child(odd) {\nbackground-color: #f2f2f2;\n}\n");
                        export_data.push_str("tr:nth-child(even) {\nbackground-color: #ddd;\n}\n");
                        export_data.push_str("</style>\n");
                        export_data.push_str("</head>\n<body>\n");
                        export_data.push_str("<h1>Transactions for Date: ");
                        export_data.push_str(selected_date);
                        export_data.push_str("</h1>\n");
                        export_data.push_str("<table>\n<tr><th>Name</th><th>Amount</th><th>Date</th><th>Category</th></tr>\n");

                        for transaction in transactions.iter() {
                            if transaction.date.trim() == *selected_date {
                                export_data.push_str("<tr>");
                                export_data.push_str(&format!("<td>{}</td>", transaction.name));
                                export_data.push_str(&format!("<td>{}</td>", transaction.amount));
                                export_data.push_str(&format!("<td>{}</td>", transaction.date));
                                export_data.push_str(&format!("<td>{}</td>", transaction.category));
                                export_data.push_str("</tr>\n");
                                if transaction.is_income {
                                    total_income += transaction.amount;
                                } else {
                                    total_expense += transaction.amount;
                                }
                            }
                        }
                        export_data.push_str("<tr>");
                        export_data.push_str("<td>Net amount (Total income - Total expense)</td>");
                        export_data.push_str(&format!(
                            "<td>{} - {} = {}",
                            total_income,
                            total_expense.abs(),
                            (total_income + total_expense)
                        ));
                        export_data.push_str("</tr>\n");
                        export_data.push_str("</table>\n");
                    } else {
                        println!("Invalid date selection.");
                        return;
                    }
                }
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    return;
                }
            }
        }

        "m" => {
            let unique_months: Vec<String> = transactions
                .iter()
                .map(|transaction| {
                    let date_parts: Vec<&str> = transaction.date.trim().split('/').collect();
                    if date_parts.len() >= 2 {
                        format!("{}/{}", date_parts[1], date_parts[2])
                    } else {
                        "".to_string()
                    }
                })
                .collect();

            let mut unique_months = unique_months
                .into_iter()
                .collect::<std::collections::HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();
            unique_months.sort();

            println!("Available months to choose from:");
            for (index, month) in unique_months.iter().enumerate() {
                println!("{}: {}", index + 1, month);
            }

            println!("Enter the number of the month you want to export (mm/yyyy): ");
            let month_index = get_user_input("").trim().parse::<usize>();

            match month_index {
                Ok(index) => {
                    if index > 0 && index <= unique_months.len() {
                        let selected_month = &unique_months[index - 1];
                        filename = format!("export_{}.html", selected_month.replace("/", "_"));
                        export_data
                            .push_str("<html>\n<head>\n<title>Income and Expense Report</title>\n");
                        export_data.push_str("<style>\n");
                        export_data
                            .push_str("table {\nborder-collapse: collapse;\nwidth: 100%;\n}\n");
                        export_data.push_str("table, th, td {\nborder: 1px solid black;\n}\n");
                        export_data.push_str("th, td {\npadding: 10px;\ntext-align: left;\n}\n");
                        export_data.push_str("th {\nbackground-color: #333;\ncolor: white;\n}\n");
                        export_data
                            .push_str("tr:nth-child(odd) {\nbackground-color: #f2f2f2;\n}\n");
                        export_data.push_str("tr:nth-child(even) {\nbackground-color: #ddd;\n}\n");
                        export_data.push_str("</style>\n");
                        export_data.push_str("</head>\n<body>\n");
                        export_data.push_str("<h1>Transactions for Month: ");
                        export_data.push_str(selected_month);
                        export_data.push_str("</h1>\n");
                        export_data.push_str("<table>\n<tr><th>Name</th><th>Amount</th><th>Date</th><th>Category</th></tr>\n");

                        for transaction in transactions.iter() {
                            let transaction_month =
                                if transaction.date.trim().contains(selected_month) {
                                    export_data.push_str("<tr>");
                                    export_data.push_str(&format!("<td>{}</td>", transaction.name));
                                    export_data
                                        .push_str(&format!("<td>{}</td>", transaction.amount));
                                    export_data.push_str(&format!("<td>{}</td>", transaction.date));
                                    export_data
                                        .push_str(&format!("<td>{}</td>", transaction.category));
                                    export_data.push_str("</tr>\n");
                                    if transaction.is_income {
                                        total_income += transaction.amount;
                                    } else {
                                        total_expense += transaction.amount;
                                    }
                                };
                        }
                        export_data.push_str("<tr>");
                        export_data.push_str("<td>Net amount (Total income - Total expense)</td>");
                        export_data.push_str(&format!(
                            "<td>{} - {} = {}",
                            total_income,
                            total_expense.abs(),
                            (total_income + total_expense)
                        ));
                        export_data.push_str("</tr>\n");
                        export_data.push_str("</table>\n");
                    } else {
                        println!("Invalid month selection.");
                        return;
                    }
                }
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    return;
                }
            }
        }

        "y" => {
            let unique_years: Vec<String> = transactions
                .iter()
                .map(|transaction| {
                    let date_parts: Vec<&str> = transaction.date.trim().split('/').collect();
                    if date_parts.len() >= 3 {
                        date_parts[2].to_string()
                    } else {
                        "".to_string()
                    }
                })
                .collect();

            let mut unique_years = unique_years
                .into_iter()
                .collect::<std::collections::HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();
            unique_years.sort();

            println!("Available years to choose from:");
            for (index, year) in unique_years.iter().enumerate() {
                println!("{}: {}", index + 1, year);
            }

            println!("Enter the index of the year you want to export: ");
            let year_index = get_user_input("").trim().parse::<usize>();

            match year_index {
                Ok(index) => {
                    if index > 0 && index <= unique_years.len() {
                        let selected_year = &unique_years[index - 1];
                        filename = format!("export_{}.html", selected_year);
                        export_data
                            .push_str("<html>\n<head>\n<title>Income and Expense Report</title>\n");
                        export_data.push_str("<style>\n");
                        export_data
                            .push_str("table {\nborder-collapse: collapse;\nwidth: 100%;\n}\n");
                        export_data.push_str("table, th, td {\nborder: 1px solid black;\n}\n");
                        export_data.push_str("th, td {\npadding: 10px;\ntext-align: left;\n}\n");
                        export_data.push_str("th {\nbackground-color: #333;\ncolor: white;\n}\n");
                        export_data
                            .push_str("tr:nth-child(odd) {\nbackground-color: #f2f2f2;\n}\n");
                        export_data.push_str("tr:nth-child(even) {\nbackground-color: #ddd;\n}\n");
                        export_data.push_str("</style>\n");
                        export_data.push_str("</head>\n<body>\n");
                        export_data.push_str("<h1>Transactions for Year: ");
                        export_data.push_str(selected_year);
                        export_data.push_str("</h1>\n");
                        export_data.push_str("<table>\n<tr><th>Name</th><th>Amount</th><th>Date</th><th>Category</th></tr>\n");

                        for transaction in transactions.iter() {
                            if transaction.date.trim().ends_with(selected_year) {
                                export_data.push_str("<tr>");
                                export_data.push_str(&format!("<td>{}</td>", transaction.name));
                                export_data.push_str(&format!("<td>{}</td>", transaction.amount));
                                export_data.push_str(&format!("<td>{}</td>", transaction.date));
                                export_data.push_str(&format!("<td>{}</td>", transaction.category));
                                export_data.push_str("</tr>\n");
                                if transaction.is_income {
                                    total_income += transaction.amount;
                                } else {
                                    total_expense += transaction.amount;
                                }
                            }
                        }
                        export_data.push_str("<tr>");
                        export_data.push_str("<td>Net amount (Total income - Total expense)</td>");
                        export_data.push_str(&format!(
                            "<td>{} - {} = {}",
                            total_income,
                            total_expense.abs(),
                            (total_income + total_expense)
                        ));
                        export_data.push_str("</tr>\n");
                        export_data.push_str("</table>\n");
                    } else {
                        println!("Invalid year selection.");
                        return;
                    }
                }
                Err(_) => {
                    println!("Invalid input. Please enter a valid number.");
                    return;
                }
            }
        }
        _ => {
            println!("Invalid input, must be day(d), month(m), year(y)")
        }
    }
    export_data.push_str("</body>\n</html>");

    match fs::write(&filename, export_data) {
        Ok(_) => println!("Data exported to {}", filename),
        Err(err) => eprintln!("Failed to export data: {}", err),
    }
    return;
}
