//! Submodule defining a task to be tracked.

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
/// A task to be tracked.
pub struct Task {
    /// The name of the task.
    name: String,
    /// The start time of the task.
    start: chrono::NaiveDateTime,
}

impl Task {
    /// Create a new task with the given name.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::task::Task;
    ///
    /// let task = Task::new("My Task");
    /// assert_eq!(task.name(), "My Task");
    ///
    /// // Using From trait
    /// let task2: Task = "My Task".into();
    /// assert_eq!(task2.name(), "My Task");
    ///
    /// let task3: Task = String::from("My Task").into();
    /// assert_eq!(task3.name(), "My Task");
    /// ```
    pub fn new<S: ToString + ?Sized>(name: &S) -> Self {
        Self { name: name.to_string(), start: chrono::Local::now().naive_local() }
    }

    /// Returns the name of the task.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::task::Task;
    ///
    /// let task = Task::new("My Task");
    /// assert_eq!(task.name(), "My Task");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    /// Marks the task as completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::task::Task;
    ///
    /// let task = Task::new("My Task");
    /// let completed = task.complete();
    /// assert_eq!(completed.name(), "My Task");
    /// ```
    pub fn complete(self) -> CompletedTask {
        CompletedTask {
            name: self.name,
            start: self.start,
            end: chrono::Local::now().naive_local(),
        }
    }
}

impl From<&str> for Task {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for Task {
    fn from(name: String) -> Self {
        Self::new(&name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Hash)]
/// A completed task.
pub struct CompletedTask {
    /// The name of the task.
    pub(crate) name: String,
    /// The start time of the task.
    pub(crate) start: chrono::NaiveDateTime,
    /// The end time of the task.
    pub(crate) end: chrono::NaiveDateTime,
}

impl CompletedTask {
    #[must_use]
    /// Returns the name of the task.
    ///
    /// # Examples
    ///
    /// ```
    /// use time_requirements::task::Task;
    ///
    /// let task = Task::new("My Task");
    /// let completed = task.complete();
    /// assert_eq!(completed.name(), "My Task");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    /// Returns the time required to complete the task.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{thread, time::Duration};
    ///
    /// use time_requirements::task::Task;
    ///
    /// let task = Task::new("My Task");
    /// thread::sleep(Duration::from_millis(10));
    /// let completed = task.complete();
    /// let time = completed.time();
    /// assert!(time.num_milliseconds() >= 10);
    /// assert!(time.num_milliseconds() > 0);
    /// ```
    pub fn time(&self) -> chrono::TimeDelta {
        self.end - self.start
    }

    /// Extends the completed task by another completed task.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{thread, time::Duration};
    ///
    /// use time_requirements::task::Task;
    ///
    /// let task1 = Task::new("Task 1");
    /// thread::sleep(Duration::from_millis(10));
    /// let mut completed1 = task1.complete();
    /// let original_time = completed1.time();
    ///
    /// let task2 = Task::new("Task 2");
    /// thread::sleep(Duration::from_millis(10));
    /// let completed2 = task2.complete();
    /// completed1.extend(&completed2);
    /// assert!(completed1.time() > original_time);
    ///
    /// // Extend again
    /// let task3 = Task::new("Task 3");
    /// thread::sleep(Duration::from_millis(5));
    /// let completed3 = task3.complete();
    /// let before_second_extend = completed1.time();
    /// completed1.extend(&completed3);
    /// assert!(completed1.time() > before_second_extend);
    ///
    /// // Comparison using PartialOrd
    /// assert!(completed2 > completed3); // completed2 took longer
    /// ```
    pub fn extend(&mut self, other: &CompletedTask) {
        self.end += other.time();
    }

    /// Returns the most precise percentage over the provided `TimeDelta`.
    ///
    /// # Arguments
    ///
    /// * `total_time` - The total time to calculate the percentage over.
    ///
    /// # Implementation Note
    ///
    /// This methods attempts to use the most precise method available to
    /// calculate the percentage. It first tries to use nanoseconds, then
    /// microseconds, then milliseconds, and finally seconds, depending on
    /// whether the conversion is lossless.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::{thread, time::Duration};
    ///
    /// use chrono::TimeDelta;
    /// use time_requirements::task::Task;
    ///
    /// let task = Task::new("Task");
    /// thread::sleep(Duration::from_millis(100));
    /// let completed = task.complete();
    ///
    /// // Total time larger than task time
    /// let total_time = TimeDelta::milliseconds(200);
    /// let percentage = completed.precise_percentage_over(total_time);
    /// assert!(percentage > 40.0 && percentage < 60.0);
    ///
    /// // Total time equal to task time
    /// let total_time = completed.time();
    /// let percentage = completed.precise_percentage_over(total_time);
    /// assert!((percentage - 100.0).abs() < 0.01);
    ///
    /// // Total time smaller than task time
    /// let total_time = TimeDelta::milliseconds(50);
    /// let percentage = completed.precise_percentage_over(total_time);
    /// assert!(percentage > 100.0);
    /// ```
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn precise_percentage_over(&self, total_time: chrono::TimeDelta) -> f64 {
        if let Some(micros) = self.time().num_microseconds()
            && let Some(total_micros) = total_time.num_microseconds()
        {
            return micros as f64 / total_micros as f64 * 100.0;
        }
        self.time().num_milliseconds() as f64 / total_time.num_milliseconds() as f64 * 100.0
    }
}

impl From<Task> for CompletedTask {
    fn from(task: Task) -> Self {
        task.complete()
    }
}

impl Ord for CompletedTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time().cmp(&other.time())
    }
}

impl PartialOrd for CompletedTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
