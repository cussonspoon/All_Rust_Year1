use druid::kurbo::Insets;
use druid::widget::{Button, Column, Label, Plot, Tabs, WidgetExt};
use druid::{AppLauncher, Data, Lens, LensExt, PlatformError, Widget, WindowDesc};
use plotters::prelude::*;
use prettytable::{cell, format, row, Table};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone, Data, Lens)]
struct AppState {
    current_tab: Tab,
    cpu_data: Vec<(u32, f64)>,
    memory_data: Vec<(u32, f64)>,
    disk_data: Vec<(u32, Vec<Option<f64>>)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tab {
    Overall,
    CPU,
    Memory,
    Disk,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Overall
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui)
        .title("System status display")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        current_tab: Tab::Overall,
        cpu_data: vec![],
        memory_data: vec![],
        disk_data: vec![],
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch the application");

    Ok(())
}

fn build_ui() -> impl Widget<AppState> {
    let cpu_plot = Plot::new(LineSeries::new(
        Vec::new().lens(AppState::cpu_data),
        |&x| x as f64,
        |&y| y,
    ))
    .fix_aspect(1.0)
    .frame(Insets::uniform(10.0))
    .x_label("Time (s)")
    .y_label("CPU Usage (%)");

    let memory_plot = Plot::new(LineSeries::new(
        Vec::new().lens(AppState::memory_data),
        |&x| x as f64,
        |&y| y,
    ))
    .fix_aspect(1.0)
    .frame(Insets::uniform(10.0))
    .x_label("Time (s)")
    .y_label("Memory Usage (%)");

    let disk_table = Table::new(
        AppState::disk_data,
        vec![
            TableConfig::with_shape(format::FormatBuilder::new().column_separator(' ').padding(1, 1)),
            TableConfig::with_shape(format::FormatBuilder::new().column_separator(' ').padding(1, 1)),
            TableConfig::with_shape(format::FormatBuilder::new().column_separator(' ').padding(1, 1)),
            TableConfig::with_shape(format::FormatBuilder::new().column_separator(' ').padding(1, 1)),
            TableConfig::with_shape(format::FormatBuilder::new().column_separator(' ').padding(1, 1)),
        ],
    )
    .fix_aspect(1.0)
    .frame(Insets::uniform(10.0));

    let tab_bar = Tabs::new()
        .with_tab("Overall", Tab::Overall)
        .with_tab("CPU", Tab::CPU)
        .with_tab("Memory", Tab::Memory)
        .with_tab("Disk", Tab::Disk);

    let tab_content = |tab: &Tab| match tab {
        Tab::Overall => {
            let label = Label::new("Overall Page Content");
            Box::new(label)
        }
        Tab::CPU => {
            let cpu_info_label = Label::new("").lens(AppState::cpu_data.map(|data| {
                if let Some(last) = data.last() {
                    format!("Current CPU usage = {:.2}%", last.1)
                } else {
                    String::from("Current CPU usage = N/A")
                }
            }));

            let content = Column::new().with_child(cpu_plot).with_child(cpu_info_label);
            Box::new(content)
        }
        Tab::Memory => {
            let memory_info_label = Label::new("").lens(AppState::memory_data.map(|data| {
                if let Some(last) = data.last() {
                    format!(
                        "Ram Usage = {:.2} GB / {:.2} GB ({:.2}%)",
                        last.1 / 1024.0,
                        16.0, // Replace with the total memory size
                        last.1 / 16.0 * 100.0
                    )
                } else {
                    String::from("Ram Usage = N/A")
                }
            }));

            let content = Column::new().with_child(memory_plot).with_child(memory_info_label);
            Box::new(content)
        }
        Tab::Disk => {
            let disk_info_label = Label::new("").lens(AppState::disk_data.map(|data| {
                let mut table = Table::new(vec![], vec![]);

                for row in data.iter() {
                    let mut r = row!(row.0.to_string());
                    for cell in row.1.iter() {
                        r.add_cell(cell!(match cell {
                            Some(value) => format!("{:.2}%", value),
                            None => String::from("N/A"),
                        }));
                    }
                    table.add_row(r);
                }

                table.to_string()
            }));

            let content = Column::new().with_child(disk_table).with_child(disk_info_label);
            Box::new(content)
        }
    };

    Column::new()
        .with_child(tab_bar)
        .with_child(
            Tab::default()
                .lens(AppState::current_tab)
                .display_fn(tab_content),
        )
}
