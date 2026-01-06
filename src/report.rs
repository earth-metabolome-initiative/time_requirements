//! Submodule defining the structs and methods for generating a report.

use std::{io::Write, path::Path};

use chrono_humanize::{Accuracy, HumanTime, Tense};
use tabled::{Table, Tabled, settings::Style};

use crate::{prelude::TimeTracker, task::CompletedTask};

/// A report for a time tracker.
pub struct Report {
    /// The time tracker to generate a report for.
    time_tracker: TimeTracker,
}

#[derive(Tabled)]
struct TableRow<'a> {
    name: &'a str,
    time: String,
    percentage: String,
}

impl Report {
    fn title(&self, depth: usize) -> String {
        format!("{} Time Report for {}\n\n", "#".repeat(depth + 1), self.time_tracker.name())
    }

    fn description(&self) -> String {
        let total_time = self.time_tracker.total_time();

        format!(
            "The total time spent on all tasks was {}.\n",
            HumanTime::from(total_time).to_text_en(Accuracy::Rough, Tense::Present),
        )
    }

    #[allow(clippy::cast_precision_loss)]
    fn slowest_task_description(&self) -> Option<String> {
        self.time_tracker.slowest_task().map(|task| {
            let total_time = self.time_tracker.total_time();
            format!(
                "The slowest task was `{}` which took {} ({:.2}% of all time).",
                task.name(),
                HumanTime::from(task.time()).to_text_en(Accuracy::Precise, Tense::Present),
                task.precise_percentage_over(total_time),
            )
        })
    }

    #[must_use]
    /// Slowest task in the report
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{thread, time::Duration};
    ///
    /// use time_requirements::prelude::*;
    ///
    /// // No tasks
    /// let tracker = TimeTracker::new("Project");
    /// let report: Report = tracker.into();
    /// assert!(report.slowest_task().is_none());
    ///
    /// // One task
    /// let mut tracker = TimeTracker::new("Project");
    /// let task = Task::new("Only task");
    /// thread::sleep(Duration::from_millis(10));
    /// tracker.add_completed_task(task);
    /// let report: Report = tracker.into();
    /// assert_eq!(report.slowest_task().unwrap().name(), "Only task");
    ///
    /// // Multiple tasks
    /// let mut tracker = TimeTracker::new("Project");
    /// let task1 = Task::new("Short task");
    /// thread::sleep(Duration::from_millis(10));
    /// tracker.add_completed_task(task1);
    ///
    /// let task2 = Task::new("Long task");
    /// thread::sleep(Duration::from_millis(100));
    /// tracker.add_completed_task(task2);
    ///
    /// let report: Report = tracker.into();
    /// assert_eq!(report.slowest_task().unwrap().name(), "Long task");
    /// ```
    pub fn slowest_task(&self) -> Option<&CompletedTask> {
        self.time_tracker.slowest_task()
    }

    /// Returns an iterator over the sub-reports.
    fn sub_reports(&self) -> impl Iterator<Item = Report> + '_ {
        self.time_tracker.sub_trackers().iter().cloned().map(|time_tracker| Self { time_tracker })
    }

    #[allow(clippy::cast_precision_loss)]
    /// Returns the text of the report.
    fn text(&self, depth: usize) -> String {
        let total_time = self.time_tracker.total_time();
        let rows = self.time_tracker.tasks().map(|task| {
            TableRow {
                name: task.name(),
                time: HumanTime::from(task.time()).to_text_en(Accuracy::Precise, Tense::Present),
                percentage: format!("{:.2}%", task.precise_percentage_over(total_time)),
            }
        });
        let mut table = Table::new(rows);
        table.with(Style::markdown());

        let mut report = String::new();

        report.push_str(&self.title(depth));
        report.push_str(&self.description());

        if let Some(description) = self.slowest_task_description() {
            report.push_str(&description);
        }

        report.push_str("\n\n");
        report.push_str(&table.to_string());

        for sub_report in self.sub_reports() {
            report.push_str("\n\n");
            report.push_str(&sub_report.text((depth + 1).min(6)));
        }

        report
    }

    /// Writes out the markdown report to a given file.
    ///
    /// # Arguments
    ///
    /// * `report_path` - The path to the file to write the report to.
    ///
    /// # Errors
    ///
    /// If the file cannot be created or written to, an error will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{path::Path, thread, time::Duration};
    ///
    /// use time_requirements::prelude::*;
    ///
    /// // Create current tracker with tasks and sub-tracker
    /// let mut current_tracker = TimeTracker::new("Current Project");
    /// let task = Task::new("Main Task");
    /// thread::sleep(Duration::from_millis(100));
    /// current_tracker.add_completed_task(task);
    ///
    /// // Add a sub-tracker
    /// let sub_tracker = TimeTracker::new("Sub Project");
    /// current_tracker.extend(sub_tracker);
    ///
    /// // Create previous tracker with same task time
    /// let mut previous_tracker = TimeTracker::new("Previous Project");
    /// let prev_task = Task::new("Main Task");
    /// thread::sleep(Duration::from_millis(100)); // Same time
    /// previous_tracker.add_completed_task(prev_task);
    ///
    /// let mut report: Report = current_tracker.into();
    ///
    /// let temp_path = std::env::temp_dir().join("test_report.md");
    /// report.write(&temp_path).expect("Failed to write report");
    /// assert!(temp_path.exists());
    /// std::fs::remove_file(temp_path).ok(); // Clean up
    /// ```
    pub fn write<S: AsRef<Path> + ?Sized>(&self, report_path: &S) -> std::io::Result<()> {
        let mut file = std::fs::File::create(report_path)?;

        writeln!(file, "{}", self.text(0))?;

        Ok(())
    }
}

impl From<TimeTracker> for Report {
    /// Creates a new report from a time tracker.
    fn from(time_tracker: TimeTracker) -> Self {
        Self { time_tracker }
    }
}
