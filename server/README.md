# Styling

Styles are Tailwind v4, with the style.css file being generated via
the Tailwind CLI. If you have the standalone version of the Tailwind
CLI installed, the build.sh script can be used to invoke it more
easily. If not, the details may be different but the script still
shows the general idea.

# Interactivity

HTMX allows for interactivity and partial page updates with
considerably less complexity and overhead than client-side rendering
frameworks. The index page and the SSE example both demonstrate HTMX
usage.

# Rust

The primary interface to the rust compiler is the `cargo` command. You
can use `cargo run` to compile and execute the server, `cargo check`
to list compilation errors and warnings without running the full
compile process, `cargo add DEP` to add a dependency, and so on.

# Database

Before trying to compile or run the code for the first time, run
`cargo install sqlx-cli` and then `cargo sqlx database setup` to
create the database and run the migrations to configure it.

If further migrations are added in the future, they can be applied by
running `cargo sqlx migrate run`

We're using SQLite for server-side data storage, and the
[SQLx](https://crates.io/crates/sqlx) toolkit to access it. SQLx has
the very useful feature of checking our SQL code when the Rust
compiler runs, which prevents a lot of potential bugs, though at the
cost of requiring a valid database setup to be in place before we can
even compile the code. It also sometimes needs additional information
in order to properly interpret the SQL. Worth it, though.


---

## ✅ SQLite Setup & SQLx CLI (New Developer Guide)

This project uses **SQLite** for the database and **SQLx** for migrations and database access.

---

### 1. Install `sqlx-cli`

`sqlx-cli` is a command line tool used to manage migrations and run database commands.

```bash
cargo install sqlx-cli
```

Verify the installation:

```bash
sqlx --version
```

---

### 2. Set the database URL

SQLx needs to know where your SQLite database file is located.
Set the environment variable **in your terminal session**:

```bash
export DATABASE_URL=sqlite://data.db
```

> ⚠️ Important: The URL **must include `sqlite://`**.
> `sqlite:data.db` will not work reliably.

---

### 3. Create the database file (if needed)

If the database file does not exist, create it:

```bash
touch data.db
```

---

### 4. Run migrations

```bash
sqlx migrate run
```

---

### 5. Start the server

```bash
cargo run
```

---

## 🧰 Debugging Tips (Common Issues)

### ❌ Error: `unable to open database file (code: 14)`

This means SQLx cannot open or create the SQLite file.

Common causes:

#### 1. `DATABASE_URL` is not set

```bash
echo $DATABASE_URL
```

If it’s empty, set it again:

```bash
export DATABASE_URL=sqlite://data.db
```

#### 2. You are in the wrong directory

Make sure you are in the project root (where `Cargo.toml` lives):

```bash
pwd
ls -la
```

#### 3. The database file does not exist or is not writable

```bash
touch data.db
chmod 664 data.db
```

---

## ⚡ Quick Start Summary

```bash
cargo install sqlx-cli
export DATABASE_URL=sqlite://data.db
touch data.db
sqlx migrate run
cargo run
```
