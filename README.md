# dEt — Data Extraction Tool

> **Extract → Cleanse/Transform (in SQL) → Load. Local-first, powered by Apache DataFusion.**
> Jinja-like templating (Rust, e.g., MiniJinja) for `config()`, `ref()`, `source()`, `var()`, `env()`.

### Project Status & Intent

**Early/empty scaffold.** This is a personal project to sharpen skills in **data engineering**, **Rust**, and **systems programming**. It’s still a plan and under active exploration—core features will be built soon.

---

## What is dEt?

**dEt** is a runner-centric data tool (inspired by PipelineWise & dbt) that **extracts** from sources, lets you **cleanse/transform using SQL** with **Apache DataFusion** (engine is always DataFusion), and **loads** the results into **one or multiple destinations**.

* **Extraction-first** with adapters (files, object storage, DBs).
* **SQL transforms** executed by **DataFusion** (columnar, parallel).
* **Multi-destination loads** (e.g., Parquet + Postgres in one run).
* **Local/CI friendly**: laptop, container, VM—no warehouse lock-in.
* **Planned** optional **Ballista** integration for distributed execution.

---

## Quick Config (`det.yaml`)

```yaml
# det.yaml
name: my_pipeline
profile: local

extract:
  sources:
    # Logical names → connectors; referenced via {{ source('<name>') }}
    order:
      type: filesystem
      format: parquet
      path: ./data/raw/orders/*.parquet

transform:
  engine: datafusion          # fixed (always DataFusion)
  sql_paths: ["models"]       # templated SQL files

load:
  destinations:
    - name: warehouse_parquet
      type: filesystem
      base_dir: ./data/warehouse
      format: parquet

    - name: analytics_pg
      type: postgres
      dsn: "postgresql://user:pass@localhost:5432/analytics"
      write_mode: merge
      schema: public
```

---

## Templated SQL (Jinja-first)

`models/stg_orders.sql`

```sql
{{ config(
  materialized="table",
  write_to=["warehouse_parquet", "analytics_pg"],  # one or many destinations
  unique_key="order_id"                             # for merge when supported
) }}

{% set days_back = var("days_back", 7) %}

WITH src AS (
  SELECT *
  FROM {{ source("order") }}
  WHERE event_time >= NOW() - INTERVAL '{{ days_back }} DAY'
),
clean AS (
  SELECT
    CAST(order_id AS BIGINT)      AS order_id,
    CAST(order_date AS DATE)      AS order_date,
    TRIM(customer_email)          AS customer_email,
    CAST(total_amount AS DOUBLE)  AS total_amount
  FROM src
  WHERE order_status = 'completed'
)
SELECT * FROM clean;
```

`models/fct_sales.sql`

```sql
{{ config(materialized="table", write_to=["warehouse_parquet"]) }}

SELECT
  DATE_TRUNC('day', order_date) AS ds,
  COUNT(*)                      AS orders,
  SUM(total_amount)             AS gross_sales
FROM {{ ref("stg_orders") }}
GROUP BY 1
ORDER BY 1;
```

---

## Built-ins (templating)

* `{{ config(**kwargs) }}` — captures per-model settings; renders nothing.
* `{{ source("name") }}` — resolves a configured source/provider.
* `{{ ref("model_name") }}` — dependency resolution & compiled name.
* `{{ var("key", default) }}` — pipeline variables (CLI/env/config).
* `{{ env("NAME", default) }}` — environment variables.
* Control flow: `{% if %}`, `{% for %}`, `{% set %}`.

> Rust plan: implement with **MiniJinja**. Flow → read SQL, render with helpers, collect `config()`, register sources, execute on **DataFusion**, route outputs to destinations.

---

## CLI (planned)

```bash
det run                           # extract → DataFusion SQL → load (all destinations)
det run --select marts.*          # run subset
det run --dest analytics_pg       # load override: single destination
det run --var days_back=3         # pass template vars
det docs                          # (future) generate DAG/docs artifacts
```

---

## Roadmap (short)

* [ ] Core runner: compile & execute templated SQL on DataFusion
* [ ] Adapters: Filesystem (Parquet/CSV/JSON), Postgres, ClickHouse, S3/MinIO/GCS
* [ ] Multi-destination loads (per-model `write_to`)
* [ ] Incremental strategies & partitions
* [ ] Basic tests & artifacts (manifest/DAG)
* [ ] Optional: **Ballista** for distributed execution

---

## License

MIT (planned).
