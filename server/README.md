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
