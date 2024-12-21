# Rust Borrow Checker Bug

This repository demonstrates a common error encountered when working with vectors and references in Rust.  The `bug.rs` file contains code that causes a runtime panic due to a borrow checker violation.  The `bugSolution.rs` file provides a corrected version that avoids this error.

**Problem:** The original code creates a reference to an element in a vector (`&vec[0]`). Afterwards, the vector is modified by adding another element. This violates Rust's borrowing rules because the reference becomes invalid after the vector is resized. 

**Solution:** Several approaches can solve this problem, including cloning the referenced value or restructuring the code to avoid mutable borrowing after creating a reference.