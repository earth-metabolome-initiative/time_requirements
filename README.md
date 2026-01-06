# Time requirements

[![CI](https://github.com/earth-metabolome-initiative/time_requirements/workflows/Rust%20CI/badge.svg)](https://github.com/earth-metabolome-initiative/time_requirements/actions)
[![Security Audit](https://github.com/earth-metabolome-initiative/time_requirements/workflows/Security%20Audit/badge.svg)](https://github.com/earth-metabolome-initiative/time_requirements/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/time_requirements/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/time_requirements)

Simple crate to measure time requirements of steps in your code.

Within our projects, we use this tool primarily to understand which parts of the build process
are slow and need to be optimized.

## Usage

```rust
use time_requirements::prelude::*;

let mut tracker = TimeTracker::new("My Project");
let task = Task::new("Build");
tracker.add_completed_task(task);

let mut sub_tracker = TimeTracker::new("Sub Task");
let sub_task = Task::new("Compile");
sub_tracker.add_completed_task(sub_task);
tracker.extend(sub_tracker);

tracker.write("report.md").unwrap();
```

This creates a markdown report of the time spent on tasks.
