# 🦀 Rust Exercises — Daily Practice Repository

Welcome to the official Rust exercises repository for the course.
This repo contains hands-on exercises for **Part 1** and **Part 2**
of the Rust curriculum, covering everything from variables and
functions to ownership, borrowing, structs, enums, and pattern
matching.

---

## 📚 How This Repo Is Organized

```
rust-exercises/
├── part1/      ← Basics: variables, types, loops, conditionals, functions
├── part2/      ← Intermediate: ownership, borrowing, strings, vectors,
│                 structs, enums, pattern matching
└── students/   ← YOUR personal folder lives here
```

---

## 🚀 Getting Started (Read This First)

### Step 1 — Fork This Repository
Click the **Fork** button at the top-right of this page.
This creates your own copy of the repo under your GitHub account.

### Step 2 — Clone Your Fork
```bash
git clone https://github.com/YOUR_USERNAME/rust-exercises.git
cd rust-exercises
```

### Step 3 — Add the Upstream Remote
This connects your local clone to the original repo so you can
always pull the latest exercises.
```bash
git remote add upstream https://github.com/INSTRUCTOR_USERNAME/rust-exercises.git
```

Verify you have both remotes:
```bash
git remote -v
# origin    https://github.com/YOUR_USERNAME/rust-exercises.git (fetch)
# origin    https://github.com/YOUR_USERNAME/rust-exercises.git (push)
# upstream  https://github.com/INSTRUCTOR_USERNAME/rust-exercises.git (fetch)
# upstream  https://github.com/INSTRUCTOR_USERNAME/rust-exercises.git (push)
```

### Step 4 — Create Your Student Folder
```bash
mkdir students/your_github_username
```
> ⚠️ Use your **exact** GitHub username. Lowercase. No spaces.

### Step 5 — Read CONTRIBUTING.md
Before submitting anything, read the
[CONTRIBUTING.md](./CONTRIBUTING.md) guide fully.

---

## 📋 Exercise Schedule

| Day | Topic | Folder |
|-----|-------|--------|
| 1 | Variables | `part1/01_variables/` |
| 2 | Data Types | `part1/02_data_types/` |
| 3 | Loops | `part1/03_loops/` |
| 4 | Conditionals | `part1/04_conditionals/` |
| 5 | Functions | `part1/05_functions/` |
| 6 | Ownership | `part2/01_ownership/` |
| 7 | Borrowing | `part2/02_borrowing/` |
| 8 | Strings | `part2/03_strings/` |
| 9 | Vectors | `part2/04_vectors/` |
| 10 | Structs | `part2/05_structs/` |
| 11 | Enums | `part2/06_enums/` |
| 12 | Pattern Matching | `part2/07_pattern_matching/` |

---

## ✅ Submission Rules

- All your work goes inside `students/your_github_username/`
- One pull request per day
- Never edit anyone else's folder
- Always sync with upstream before starting a new exercise

---

## 🛠️ Prerequisites

- Rust installed via [rustup.rs](https://rustup.rs)
- Git installed
- A GitHub account
- VS Code with the `rust-analyzer` extension (recommended)

---

## 👨🏫 Need Help?

Open a GitHub **Issue** with the label `question`.
Tag it with the topic, e.g., `part1/ownership`.
