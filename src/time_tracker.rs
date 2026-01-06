//! Submodule defining the task tracker.

use std::path::Path;

use crate::{
    report::Report,
    task::{CompletedTask, Task},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
/// A tracker for tasks.
pub struct TimeTracker {
    /// Name of the overall project.
    name: String,
    /// The tasks being tracked.
    tasks: Vec<CompletedTask>,
    /// The sub-trackers being tracked.
    sub_trackers: Vec<TimeTracker>,
    /// Start of the project.
    start: chrono::NaiveDateTime,
}

impl TimeTracker {
    /// Creates a new time tracker for the given project name.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::time_tracker::TimeTracker;
    ///
    /// let tracker = TimeTracker::new("My Project");
    /// assert_eq!(tracker.name(), "My Project");
    /// ```
    pub fn new<S: ToString + ?Sized>(name: &S) -> Self {
        Self {
            name: name.to_string(),
            tasks: Vec::new(),
            sub_trackers: Vec::new(),
            start: chrono::Local::now().naive_local(),
        }
    }

    /// Returns the sub-trackers.
    pub(crate) fn sub_trackers(&self) -> &[TimeTracker] {
        &self.sub_trackers
    }

    /// Extends the tracker from another tracker.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::{task::Task, time_tracker::TimeTracker};
    ///
    /// let mut tracker1 = TimeTracker::new("Project 1");
    /// let task = Task::new("Task");
    /// tracker1.add_completed_task(task);
    /// let initial_tasks = tracker1.tasks().count();
    ///
    /// let tracker2 = TimeTracker::new("Project 2");
    /// tracker1.extend(tracker2);
    /// assert_eq!(tracker1.tasks().count(), initial_tasks + 1); // Adds to_completed_task of tracker2
    /// ```
    pub fn extend(&mut self, other: TimeTracker) {
        self.tasks.push(other.clone().into());
        self.sub_trackers.push(other);
    }

    /// Adds a task to the tracker.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::{task::Task, time_tracker::TimeTracker};
    ///
    /// let mut tracker = TimeTracker::new("Project");
    /// assert_eq!(tracker.tasks().count(), 0);
    ///
    /// let task1 = Task::new("Task 1");
    /// tracker.add_completed_task(task1);
    /// assert_eq!(tracker.tasks().count(), 1);
    ///
    /// let task2 = Task::new("Task 2");
    /// tracker.add_completed_task(task2);
    /// assert_eq!(tracker.tasks().count(), 2);
    /// ```
    pub fn add_completed_task(&mut self, task: Task) {
        self.tasks.push(task.into());
    }

    /// Extends a previously completed task.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{thread, time::Duration};
    ///
    /// use time_requirements::{task::Task, time_tracker::TimeTracker};
    ///
    /// let mut tracker = TimeTracker::new("Project");
    ///
    /// // Add new task
    /// let task1 = Task::new("Task");
    /// thread::sleep(Duration::from_millis(10));
    /// tracker.add_or_extend_completed_task(task1);
    /// assert_eq!(tracker.tasks().count(), 1);
    ///
    /// // Extend existing task
    /// let task2 = Task::new("Task"); // Same name
    /// thread::sleep(Duration::from_millis(10));
    /// tracker.add_or_extend_completed_task(task2);
    /// assert_eq!(tracker.tasks().count(), 1); // Still one task, but extended
    ///
    /// // Add another new task
    /// let task3 = Task::new("Another Task");
    /// tracker.add_or_extend_completed_task(task3);
    /// assert_eq!(tracker.tasks().count(), 2);
    /// ```
    pub fn add_or_extend_completed_task(&mut self, task: Task) {
        for existing_task in &mut self.tasks {
            if existing_task.name() == task.name() {
                existing_task.extend(&task.into());
                return;
            }
        }
        self.tasks.push(task.into());
    }

    #[must_use]
    /// Returns the name of the project.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::time_tracker::TimeTracker;
    ///
    /// let tracker = TimeTracker::new("My Project");
    /// assert_eq!(tracker.name(), "My Project");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    /// Returns the start time of the project.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::time_tracker::TimeTracker;
    ///
    /// let tracker = TimeTracker::new("Project");
    /// let start_time = tracker.start();
    /// // start_time is a NaiveDateTime
    /// ```
    pub fn start(&self) -> chrono::NaiveDateTime {
        self.start
    }

    /// Iterates the task from the tracker.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::{task::Task, time_tracker::TimeTracker};
    ///
    /// // Empty tracker
    /// let tracker = TimeTracker::new("Project");
    /// let tasks: Vec<_> = tracker.tasks().collect();
    /// assert!(tasks.is_empty());
    ///
    /// // Tracker with tasks
    /// let mut tracker = TimeTracker::new("Project");
    /// let task1 = Task::new("Task 1");
    /// let task2 = Task::new("Task 2");
    /// tracker.add_completed_task(task1);
    /// tracker.add_completed_task(task2);
    ///
    /// let tasks: Vec<_> = tracker.tasks().collect();
    /// assert_eq!(tasks.len(), 2);
    /// assert_eq!(tasks[0].name(), "Task 1");
    /// assert_eq!(tasks[1].name(), "Task 2");
    /// ```
    pub fn tasks(&self) -> impl Iterator<Item = &CompletedTask> {
        self.tasks.iter()
    }

    #[must_use]
    /// Returns a reference to the slowest task.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{thread, time::Duration};
    ///
    /// use time_requirements::{task::Task, time_tracker::TimeTracker};
    ///
    /// // No tasks
    /// let tracker = TimeTracker::new("Project");
    /// assert!(tracker.slowest_task().is_none());
    ///
    /// // One task
    /// let mut tracker = TimeTracker::new("Project");
    /// let task = Task::new("Only task");
    /// thread::sleep(Duration::from_millis(10));
    /// tracker.add_completed_task(task);
    /// assert_eq!(tracker.slowest_task().unwrap().name(), "Only task");
    ///
    /// // Multiple tasks
    /// let mut tracker = TimeTracker::new("Project");
    /// let task1 = Task::new("Short");
    /// thread::sleep(Duration::from_millis(10));
    /// tracker.add_completed_task(task1);
    ///
    /// let task2 = Task::new("Long");
    /// thread::sleep(Duration::from_millis(100));
    /// tracker.add_completed_task(task2);
    ///
    /// assert_eq!(tracker.slowest_task().unwrap().name(), "Long");
    /// ```
    pub fn slowest_task(&self) -> Option<&CompletedTask> {
        self.tasks.iter().max()
    }

    #[must_use]
    /// Returns the total amount of time spent on all tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{thread, time::Duration};
    ///
    /// use time_requirements::{task::Task, time_tracker::TimeTracker};
    ///
    /// // Empty tracker
    /// let tracker = TimeTracker::new("Project");
    /// assert_eq!(tracker.total_time().num_milliseconds(), 0);
    ///
    /// // Tracker with tasks
    /// let mut tracker = TimeTracker::new("Project");
    /// let task1 = Task::new("Task 1");
    /// thread::sleep(Duration::from_millis(50));
    /// tracker.add_completed_task(task1);
    ///
    /// let task2 = Task::new("Task 2");
    /// thread::sleep(Duration::from_millis(30));
    /// tracker.add_completed_task(task2);
    ///
    /// let total = tracker.total_time();
    /// assert!(total.num_milliseconds() >= 80); // At least 80ms
    /// ```
    pub fn total_time(&self) -> chrono::TimeDelta {
        self.tasks.iter().map(CompletedTask::time).sum()
    }

    /// Saves the report as a JSON in the provided directory.
    ///
    /// # Arguments
    ///
    /// * `directory` - The directory to save the report in.
    ///
    /// # Errors
    ///
    /// If the directory does not exist or is not writable, an error will be
    /// returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    ///
    /// use time_requirements::time_tracker::TimeTracker;
    ///
    /// let tracker = TimeTracker::new("Project");
    /// let temp_dir = std::env::temp_dir();
    /// tracker.save(&temp_dir).expect("Failed to save");
    /// let file_path = temp_dir.join("Project.json");
    /// assert!(file_path.exists());
    /// std::fs::remove_file(file_path).ok(); // Clean up
    /// ```
    pub fn save(&self, directory: &std::path::Path) -> std::io::Result<()> {
        let file = std::fs::File::create(directory.join(format!("{}.json", self.name)))?;
        serde_json::to_writer(file, self)?;
        Ok(())
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
    /// ```rust
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
    /// let temp_path = std::env::temp_dir().join("test_report.md");
    /// current_tracker.write(&temp_path).expect("Failed to write report");
    /// assert!(temp_path.exists());
    /// std::fs::remove_file(temp_path).ok(); // Clean up
    /// ```
    pub fn write<S: AsRef<Path> + ?Sized>(&self, report_path: &S) -> std::io::Result<()> {
        let report: Report = self.clone().into();
        report.write(report_path)?;

        Ok(())
    }
}

impl From<TimeTracker> for CompletedTask {
    fn from(tracker: TimeTracker) -> Self {
        CompletedTask {
            name: tracker.name.clone(),
            start: tracker.start,
            end: tracker.start + tracker.total_time(),
        }
    }
}
