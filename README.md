# rust-template

<div align="center">
  <!-- GitHub Profile Shield with Logo -->
  <a href="https://marcoramos17.github.io/">
    <img src="https://img.shields.io/badge/GitHub-Profile-181717?logo=github" alt="{GitHub} Profile"></a>

<!-- Rustdoc Pages Shield with Logo -->

<a href="https://marcoramos17.github.io/rust-template/rust_template/">
    <img src="https://img.shields.io/badge/docs-available-brightgreen" alt="{RustDoc} Pages"></a>
</div>

# Affiliation - Project (Template)

This repository serves as a centralized template for managing Rust programming language projects efficiently. It features optional GitHub Project support (for kanban/task tracking), flexible branching, and — most notably — **automated generation and deployment of Rust documentation via GitHub Actions and GitHub Pages**.

---

## Document Information

### Affiliation:

*Example University*

### Project:

*Module/Project Name*

### Title:

*Document Title Here*

### Author(s):

*Marco Ramos - 10415201*

### Description:

A reusable and well-structured template tailored for Rust projects. Automatically builds and hosts documentation using `cargo doc` and GitHub Pages.

---

## Key Features

✅ Clean and modular Rust project structure
✅ Automated documentation with `cargo doc`
✅ GitHub Actions integration for generating and publishing docs
✅ Hosted docs accessible via GitHub Pages
✅ Suggested branching workflow for project development
✅ GitHub Project Boards support (optional)

---

## 📄 Rust Documentation Hosting

This template includes a GitHub Actions workflow that:

1. Runs `cargo doc --no-deps` to generate documentation for the current codebase.
2. Deploys the generated documentation to the `gh-pages` branch.
3. Hosts it using GitHub Pages at:
   https://marcoramos17.github.io/`<your-repo-name>`/

> 🛠️ _Note: If docs are not building or the page isn't updating, check the status of your GitHub Actions and make sure your repository settings allow GitHub Pages from the `gh-pages` branch._

---

## Workflow Guide

### **Working with the Main Branch**

The `main` branch:

- Repository main branch

#### Key Actions:

- **Update the repository (`main` branch):**
  ```bash
  git add .
  git commit -m "message"
  git push
  ```

---

### **Creating a New Branch**

To start working on a new branch:

1. **Create a new branch OR switch to an existing branch:**

   ```bash
   git checkout -b branch-name
   #OR
   git checkout branch-name
   ```

   - Branch naming format: `-- no format specified --`. Ex: `-- branch_name --`
2. **Push the branch to the remote repository (optional):**

   ```bash
   git push
   ```

---

### **Keeping The Other Branches Updated**

If updates are made to the `main` branch, pull those changes into other branches:

1. **Switch to another document branch:**

   ```bash
   git checkout branch-name
   ```
2. **Pull changes from `main`:**

   ```bash
   git pull origin main
   ```
3. **Resolve any merge conflicts (if applicable):**
   Open conflicting files, resolve issues manually, and mark them resolved:

   ```bash
   git add resolved-file
   git commit
   ```

---

### *File Structures*

#### Comparison Table

| FEATURE                        | Grouped Modules       | Workspaces     | Nested Libraries |
| ------------------------------ | --------------------- | -------------- | ---------------- |
| **Scalability**          | Limited               | High           | Medium           |
| **Dependency Isolation** | No                    | Yes            | No               |
| **Reusability**          | Limited               | High           | Limited          |
| **Complexity**           | Low                   | High           | Medium           |
| **Best For**             | Small/Medium Projects | Large Projects | Medium Projects  |

#### Grouped Modules

```
├── src/
│   ├── main.rs                # Entry point for the binary
│   ├── lib.rs                 # Library module (shared logic)
│   ├── modules/               # Grouped modules directory
│   │   ├── group1/
│   │   │   ├── mod1.rs        # Module 1 in Group 1
│   │   │   ├── mod2.rs        # Module 2 in Group 1
│   │   ├── group2/
│   │   │   ├── mod3.rs        # Module 3 in Group 2
│   │   │   ├── mod4.rs        # Module 4 in Group 2
```

Usage Example:

```
// lib.rs
pub mod modules;

// modules/group1/mod1.rs
pub fn greet() {
    println!("Hello from Group 1, Module 1!");
}

// main.rs
mod modules;

fn main() {
    modules::group1::mod1::greet();
}
```

#### Workspaces

```
├── Cargo.toml                # Workspace-level manifest
├── src/                      # Main binary crate
│   ├── main.rs
├── module_group1/            # Library crate 1
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── feature_a.rs
│   │   ├── feature_b.rs
├── module_group2/            # Library crate 2
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── feature_c.rs
│   │   ├── feature_d.rs
```

Usage Example:

```
use module_group1::greet_from_group1;
use module_group2::greet_from_group2;

fn main() {
    greet_from_group1();
    greet_from_group2();
}

// filepath: module_group1/src/lib.rs
pub fn greet_from_group1() {
    println!("Hello from Module Group 1!");
}

// filepath: module_group2/src/lib.rs
pub fn greet_from_group2() {
    println!("Hello from Module Group 2!");
}
```

#### Nested Libraries

```
├── src/
│   ├── main.rs                # Entry point for the binary
│   ├── lib.rs                 # Main library entry point
│   ├── module_group1/         # Grouped modules directory
│   │   ├── mod.rs             # Acts like a "local lib.rs" for the group
│   │   ├── feature_a.rs
│   │   ├── feature_b.rs
│   ├── module_group2/         # Another grouped module directory
│   │   ├── mod.rs
│   │   ├── feature_c.rs
│   │   ├── feature_d.rs
```

Usage Example:

```
// lib.rs
pub mod module_group1;

// module_group1/mod.rs
pub mod feature_a;
pub mod feature_b;

// module_group1/feature_a.rs
pub fn greet() {
    println!("Hello from Feature A!");
}

// main.rs
mod module_group1;

fn main() {
    module_group1::feature_a::greet();
}
```

---

### **General Tips**

- Regularly pull updates from `main` to keep branches up-to-date with the latest template changes.
- Use descriptive branch names (e.g., `COV_6006CEM_Train-Accidents`).
- Write clear commit messages for better collaboration and tracking.

---

This README serves as an introduction to maintain an organized and efficient workflow for managing Computer Science Projects.
