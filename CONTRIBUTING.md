# 🤝 Contributing Guide

This document explains exactly how to submit your daily exercises.
Read every word. This workflow is designed so that **you will
never have a merge conflict** as long as you follow it.

---

## The Golden Rule

> **You only ever edit files inside your own folder:**
> `students/your_github_username/`
>
> Never touch any other file in the repo.
> That's it. That's the whole rule.

---

## Daily Workflow — Step by Step

Follow these steps **every single day** before you start an exercise.

---

### Step 1 — Sync Your Fork With Upstream

New exercises and fixes get added to the upstream repo regularly.
Before you write a single line of code, pull the latest changes:

```bash
# Make sure you're on the main branch
git checkout main

# Pull latest changes from the original repo (upstream)
git fetch upstream
git merge upstream/main

# Push those updates to your own fork on GitHub
git push origin main
```

> 💡 **Why this matters:** If you skip this step and someone else
> already submitted work that changed a shared file, you could
> get merge conflicts. Syncing first prevents that entirely.

---

### Step 2 — Create a New Branch for Today's Exercise

Never work directly on the `main` branch. Always create a new branch
named using this exact format:

```
your_github_username/dayNN-topic
```

Examples:
```bash
git checkout -b alice/day01-variables
git checkout -b alice/day06-ownership
git checkout -b alice/day09-vectors
```

Naming your branch with your username means it can **never conflict**
with another student's branch.

---

### Step 3 — Create Your Exercise File

Navigate to today's exercise folder and read the `exercises.md` file
inside it. For example, for Day 1:

```bash
cat part1/01_variables/exercises.md
```

Now create your solution inside **your student folder** — NOT inside
the exercise folder:

```bash
# Create the folder structure inside your student directory
mkdir -p students/your_github_username/part1/01_variables

# Create your solution file
touch students/your_github_username/part1/01_variables/solutions.rs
```

Your full folder path should look like:
```
students/
└── alice/
    └── part1/
        └── 01_variables/
            └── solutions.rs
```

---

### Step 4 — Write Your Solutions

Open the file in your editor and write your solutions.
Each exercise folder has a `exercises.md` with instructions and
a `README.md` with detailed explanations of the concept.

Write each exercise as a separate function or comment block.
Use this template at the top of every solutions file:

```rust
// ============================================
// Student: your_github_username
// Topic: Variables (Part 1, Day 1)
// Date: YYYY-MM-DD
// ============================================

// Exercise 1
fn exercise_1() {
    // Your solution here
}

// Exercise 2
fn exercise_2() {
    // Your solution here
}

// Exercise 3
fn exercise_3() {
    // Your solution here
}

fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
}
```

---

### Step 5 — Commit Your Work

```bash
# Stage only YOUR files
git add students/your_github_username/

# Commit with a clear, descriptive message
git commit -m "feat(alice): day01 - variables exercises"
```

**Commit message format:**
```
feat(your_username): dayNN - topic name
```

Examples:
```
feat(alice): day01 - variables exercises
feat(bob): day06 - ownership exercises
feat(carol): day08 - strings exercises
```

---

### Step 6 — Push Your Branch to Your Fork

```bash
git push origin your_github_username/day01-variables
```

---

### Step 7 — Open a Pull Request

1. Go to your fork on GitHub
2. Click **"Compare & pull request"** (GitHub shows this automatically
   after a push)
3. Set the **base repository** to the instructor's repo
4. Set the **base branch** to `main`
5. Set the **compare branch** to your branch
6. Fill in the Pull Request template (it will auto-populate)
7. Click **"Create pull request"**

---

### Step 8 — Wait for Review

Your PR will be reviewed. You might get feedback asking you to fix
something. If that happens:

```bash
# Make your fixes in the same branch
git add students/your_github_username/
git commit -m "fix(alice): address review comments on day01"
git push origin alice/day01-variables
```

GitHub automatically updates your open PR with the new commits.

---

## If You Ever Get a Merge Conflict

This should almost never happen if you follow this guide. But if it
does, here's how to fix it:

```bash
# On your branch, fetch and merge the latest main
git fetch upstream
git merge upstream/main
```

If there are conflicts, Git will tell you which files. Open those
files and look for the conflict markers:

```
<<<<<<< HEAD
your changes
=======
changes from main
>>>>>>> upstream/main
```

Since your code is in `students/your_username/` and nobody else
touches that folder, conflicts should only appear in shared files
you accidentally edited. The fix: keep the main branch version of
any shared files and keep your version of your student folder.

```bash
# After resolving conflicts
git add .
git commit -m "fix(alice): resolve merge conflict"
git push origin alice/day01-variables
```

---

## Pull Request Checklist

Before submitting, verify all of these:

- [ ] I synced with upstream before starting
- [ ] My branch name follows the format: `username/dayNN-topic`
- [ ] My files are inside `students/my_username/` only
- [ ] My solution file has the header comment with my name and date
- [ ] My code compiles (`rustc solutions.rs` or `cargo run`)
- [ ] I have not edited any files outside my student folder
- [ ] My commit message follows the format: `feat(username): dayNN - topic`
- [ ] I filled out the Pull Request template

---

## What NOT To Do

❌ Do not push directly to `main`
❌ Do not edit files in `part1/` or `part2/` exercise folders
❌ Do not edit another student's folder
❌ Do not open more than one PR per day exercise
❌ Do not force push to shared branches
