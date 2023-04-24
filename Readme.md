# Wrs

Wrs stands for **Wr**ike Ru**s**t. It's a Wrike CLI client helping you to search and filter tasks more faster.

⚠️ This project is under development, please use it with caution.

## Installation

Download a [releases](https://github.com/LeBaronDeCharlus/wrs/releases), and add binary in your Path.

### Usage

```shell
> wrs --help

Usage: wrs [COMMAND]

Commands:
  tasks  Actions on tasks, default list your tasks
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
