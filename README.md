# Time requirements

[![CI](https://github.com/earth-metabolome-initiative/time_requirements/workflows/Rust%20CI/badge.svg)](https://github.com/earth-metabolome-initiative/time_requirements/actions)
[![Security Audit](https://github.com/earth-metabolome-initiative/time_requirements/workflows/Security%20Audit/badge.svg)](https://github.com/earth-metabolome-initiative/time_requirements/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/time_requirements/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/time_requirements)
[![Crates.io](https://img.shields.io/crates/v/time_requirements.svg)](https://crates.io/crates/time_requirements)
[![Docs.rs](https://docs.rs/time_requirements/badge.svg)](https://docs.rs/time_requirements)

Simple crate to measure time requirements of steps in your code.

Within our projects, we use this tool primarily to understand which parts of the build process
are slow and need to be optimized.

## Usage

```rust
use std::{thread, time::Duration};
use time_requirements::prelude::*;

let mut tracker = TimeTracker::new("My Project");

// Start tracking a task
let task = Task::new("Heavy Computation");

// Simulate work
thread::sleep(Duration::from_millis(100));

// Complete the task and add it to the tracker
tracker.add_completed_task(task);

// You can also use sub-trackers for logical grouping
let mut sub_tracker = TimeTracker::new("Database Operations");
let sub_task = Task::new("Query");
// ... perform query ...
sub_tracker.add_completed_task(sub_task);

// Merge sub-tracker into the main tracker
tracker.extend(sub_tracker);

// Save the report to a file
tracker.write("report.md").unwrap();
```

This creates a markdown report of the time spent on tasks, which you can see an example of in
`report.md`.

## Features

- **Simple Task Tracking**: Measure the duration of individual tasks.
- **Hierarchical Reporting**: Use sub-trackers to group tasks logically.
- **Markdown Reports**: Automatically generate readable Markdown reports including:
  - Total time spent
  - Slowest task analysis
  - Detailed table of tasks with time and percentage distributions
  - JSON export support via `save()`
