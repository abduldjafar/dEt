# DPT — Data Processing Tool

> **SQL-first transformations, dbt-like ergonomics, local-first execution, powered by Apache DataFusion (with Ballista planned).**

DPT is a lightweight data transformation framework that lets you define your pipeline in **SQL** and run it anywhere the DPT runner can execute—your laptop, a CI job, a container, or a VM. It embraces familiar concepts like **models**, **dependencies**, **seeds**, and **incremental builds**, while using **Apache DataFusion** under the hood for fast, in-memory query execution over files and databases.

**Think dbt, but:**

* **Local/runner-centric execution** (no warehouse lock-in)
* **DataFusion** as the default engine; **Ballista integration is planned and in progress** for distributed, scale-out execution
* **File + SQL pipelines** that target Parquet/CSV/Delta or push to databases (via adapters)

---

### Project Status & Intent

* **Status:** Early-stage; APIs and structure may change.
* **Roadmap:** Ballista-based distributed execution (ongoing exploration).
* **Purpose:** A personal project to sharpen skills in **Rust**, **data engineering**, and **systems programming**.
