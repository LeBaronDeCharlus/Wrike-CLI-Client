# Wrs

Wrs stands for **Wr**ike Ru**s**t. It's a Wrike CLI client helping you to search and filter tasks more faster.

⚠️ This project is under development, please use it with caution.

## Installation

Download a [releases](https://github.com/LeBaronDeCharlus/wrs/releases), and add binary in your $PATH.

`wrs` will look for 3 env vars.
```rust
    let user: String = env::var("WRIKE_USER")?;
    let url: String = env::var("URL")?;
    let token: String = env::var("TOKEN")?;
```
Wrike user must be your `Contact ID`, see [this page](https://developers.wrike.com/api/v4/contacts/) for more information.

You need to configure and export them in your $PATH.

### Usage

```shell
> wrs --help

Usage: wrs [COMMAND]

Commands:
  tasks  Actions on tasks, default list your tasks
  workflows  Actions on workflows, default list your workflows
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

#### Tasks action

```shell
> wrs tasks --help

Usage: wrs tasks [OPTIONS]

Options:
      --search <SEARCH>  Search tasks, matching words in title
      --status <STATUS>  Filter searched tasks by status
      --folder <FOLDER>
  -m, --me               Filter by only looking for your tasks
  -h, --help             Print help
```

Searching **my** (--me) task by looking on title **search** (--search) and filter by **status** (--status).

```shell
> wrs tasks --me --search dns --status active
+------------------+-----------------+----------+----------------------------------------------+--------+
| id               | name            | priority | url                                          | status |
+------------------+-----------------+----------+----------------------------------------------+--------+
| IEABSXDCKRAXO2C7 | Check Mails DNS | Normal   | https://www.wrike.com/open.htm?id=1098344543 | Active |
+------------------+-----------------+----------+----------------------------------------------+--------+
```


#### Workflows action

```shell
> wrs workflows --help

Actions on workflows, default list your workflows

Usage: wrs workflows

Options:
  -h, --help  Print help
```

List of my workflows

```shell
> wrs workflows
+------------------+-----------------+------------------+----------------+
| id               | name            | status id        | status name    |
+------------------+-----------------+------------------+----------------+
| XXXXXXDCK4AF7RFO | Sprint Workflow | XXXXXXDCJMAF7RFO | New            |
+------------------+-----------------+------------------+----------------+
| XXXXXXDCK4AF7RFO | Sprint Workflow | XXXXXXDCJMAF7RFY | Completed      |
+------------------+-----------------+------------------+----------------+
| XXXXXXDCK4AF7TDB | Design Workflow | XXXXXXDCJMCGSU22 | To be designed |
+------------------+-----------------+------------------+----------------+
| XXXXXXDCK4AF7TDB | Design Workflow | XXXXXXDCJMB3JYA4 | To be reviewed |
+------------------+-----------------+------------------+----------------+
```
