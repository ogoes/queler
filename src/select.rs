use crate::clause::Clause;

pub struct Table(String, Option<String>);

impl From<String> for Table {
    fn from(table: String) -> Self {
        Table(table, None)
    }
}

impl From<&String> for Table {
    fn from(table: &String) -> Self {
        Table(table.into(), None)
    }
}

impl From<&'_ str> for Table {
    fn from(table: &'_ str) -> Self {
        Table(table.to_string(), None)
    }
}

impl<T: ToString, A: ToString> From<(T, A)> for Table {
    fn from((table, alias): (T, A)) -> Self {
        Table(table.to_string(), Some(alias.to_string()))
    }
}

#[derive(Default)]
pub struct SelectBuilder {
    columns: Vec<String>,
    from: Vec<Table>,
    inner_join: Vec<(Table, Clause)>,
    r#where: Vec<Clause>,
    all: bool,
}

pub struct Select {
    columns: String,
    from: String,
    inner_join: String,
    r#where: String,
}

impl SelectBuilder {
    pub fn new() -> Self {
        Self {
            all: true,
            ..Self::default()
        }
    }
}

impl SelectBuilder {
    fn _table(table: &Table) -> String {
        if let Some(alias) = &table.1 {
            format!("{} AS {}", table.0, alias)
        } else {
            table.0.clone()
        }
    }

    fn _columns(&self) -> String {
        if self.all {
            "*".into()
        } else {
            self.columns.iter().fold("".to_string(), |acc, column| {
                if acc.len() > 0usize {
                    format!("{}, {}", acc, column)
                } else {
                    column.to_owned()
                }
            })
        }
    }

    fn _from(&self) -> String {
        self.from.iter().fold("".to_string(), |acc, from| {
            if acc.len() > 0usize {
                format!("{}, {}", acc, SelectBuilder::_table(from))
            } else {
                format!("FROM {}", SelectBuilder::_table(from))
            }
        })
    }

    fn _where(&self) -> String {
        let valid: Vec<&Clause> = self.r#where.iter().filter(|clause| clause.valid).collect();

        valid.iter().fold("".to_string(), |acc, clause| {
            if !clause.valid {
                return acc;
            }

            if acc.len() > 0usize {
                format!("{} AND ({})", acc, clause.to_string())
            } else {
                let as_string = clause.to_string();

                let mut indices = as_string.char_indices();
                let (_, first) = indices.next().unwrap_or((0, '_'));
                let (_, last) = indices.last().unwrap_or((0, '_'));

                if valid.len() > 1usize {
                    if first != '(' && last != ')' {
                        format!("WHERE ({})", clause.to_string())
                    } else {
                        format!("WHERE {}", clause.to_string())
                    }
                } else {
                    format!("WHERE {}", clause.to_string())
                }
            }
        })
    }

    fn _inner_join(&self) -> String {
        self.inner_join
            .iter()
            .fold("".to_string(), |acc, (table, clause)| {
                let base = format!("{} INNER JOIN", acc);

                if !clause.valid {
                    format!("{} {}", base, SelectBuilder::_table(table))
                } else {
                    format!(
                        "{} {} ON {}",
                        base,
                        SelectBuilder::_table(table),
                        clause.to_string()
                    )
                }
            })
    }
}

impl std::fmt::Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SELECT {} {} {} {}",
            self.columns, self.from, self.inner_join, self.r#where
        )
    }
}

impl SelectBuilder {
    pub fn select<T: ToString>(&mut self, columns: &[T]) -> &mut Self {
        self.all = false;
        self.columns
            .append(&mut columns.iter().map(|t| t.to_string()).collect());
        self
    }

    pub fn from<T: Into<Table>>(&mut self, table: T) -> &mut Self {
        self.from.push(table.into());
        self
    }

    pub fn inner_join<T: Into<Table>>(&mut self, table: T, clause: Clause) -> &mut Self {
        self.inner_join.push((table.into(), clause));
        self
    }

    pub fn r#where(&mut self, clause: Clause) -> &mut Self {
        self.r#where.push(clause);
        self
    }
}

impl SelectBuilder {
    pub fn build(&mut self) -> Select {
        Select {
            columns: self._columns(),
            from: self._from(),
            r#where: self._where(),
            inner_join: self._inner_join(),
        }
    }
}
