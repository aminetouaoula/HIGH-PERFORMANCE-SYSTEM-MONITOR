use cli_system_monitor::metrics::{Collector, cpu::CpuCollector, ram::RamCollector};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::{
    cursor, execute,
    style::{self, Stylize},
    terminal::{self, ClearType},
};
use std::{
    io::{Write, stdout},
    thread,
    time::Duration,
};
use sysinfo::System;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut collectors: Vec<Box<dyn Collector>> =
        vec![Box::new(CpuCollector {}), Box::new(RamCollector {})];

    let mut sys = System::new_all();
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;

    loop {
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 1)
        )?;

        print!(
            "{}\r\n",
            "--- HIGH-PERFORMANCE SYSTEM MONITOR ---".bold().cyan()
        );

        sys.refresh_all();

        for collector in collectors.iter_mut() {
            let value = collector.collect(&sys);
            let name = collector.name();
            let bar = create_bar(value);

            print!(
                "{:<12} {} {:>6.2}%\r\n",
                name.yellow(),
                bar,
                style::style(format!("{:.2}", value)).green()
            );
        }

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q')
                    || (key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL))
                {
                    terminal::disable_raw_mode()?;
                    println!("\r\nExiting safely...");
                    break;
                }
            }
        }

        stdout.flush()?;
        thread::sleep(Duration::from_secs(2));
    }

    Ok(())
}
fn create_bar(percentage: f32) -> String {
    let width = 20;
    let filled = ((percentage / 100.0) * width as f32).min(width as f32) as usize;
    let empty = width - filled;
    format!(
        "{}{}{}{}",
        "[".white(),
        "|".repeat(filled).green(),
        ".".repeat(empty).grey(),
        "]".white()
    )
}
