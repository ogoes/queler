use crate::value;

#[derive(Clone)]
pub enum ClauseType {
    And,
    Or,
}

#[derive(Clone)]
pub struct Clause {
    pub r#type: ClauseType,
    clauses: Vec<String>,
    pub valid: bool,
}

impl Default for Clause {
    fn default() -> Self {
        Self {
            r#type: ClauseType::And,
            clauses: vec![],
            valid: false,
        }
    }
}

impl From<Clause> for value::Value {
    fn from(v: Clause) -> Self {
        value::Value::String(format!("({})", v.to_string()))
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let joiner = match self.r#type {
            ClauseType::And => " AND ",
            ClauseType::Or => " OR ",
        };

        write!(f, "{}", self.clauses.join(joiner))
    }
}

impl Clause {
    pub fn push(&mut self, value: String) {
        self.valid = true;
        self.clauses.push(value);
    }
}

impl std::fmt::Debug for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clause: {}", self)
    }
}

#[macro_export]
macro_rules! clause {
    () => { $crate::clause::Clause::default() };
    (@convert $name:expr => $value:expr) => {
        {
            let value = $crate::clause!(@to_string $value.clone());
            match $crate::value::Value::from($value) {
                $crate::value::Value::NULL => format!("{} IS NULL", $name),
                $crate::value::Value::Boolean(_) => format!("CAST({} AS INTEGER) = {}", $name, value),
                _ => format!("{} = {}", $name, value)
            }
        }
    };
    (@to_string $value:expr) => (
        $crate::value::Value::from($value).to_string()
    );
    (@to_string $clause:expr, $name:expr => ( $($ors:tt) or* )) => (

        let mut vec = vec![];
        $(
            vec.push($crate::clause!(@convert $name => $ors););
        )*

        $clause.push(format!("({})", vec.join(" OR ")));
    );
    (@to_string $clause:expr, $name:expr => ( $($ors:tt) and* )) => (

        let mut vec = vec![];
        $(
            vec.push($crate::clause!(@convert $name => $ors););
        )*

        $clause.push(format!("({})", vec.join(" AND ")));
    );
    (@to_string $clause:expr, $name:expr => $value:expr) => {
        $clause.push($crate::clause!(@convert $name => $value).clone());
    };
    (@expand $clause:expr;) => {};
    (@expand $clause:expr; $name:expr => ( $($ors:tt) or* ) $(, $tail:tt)*) => {
        $crate::clause!(@to_string $clause, $name => ( $($ors) or* ));
        $crate::clause!(@expand $clause; $($tail)*);
    };
    (@expand $clause:expr; $name:expr => ( $($ors:tt) or* ), $($tail:tt)*) => {
        $crate::clause!(@to_string $clause, $name => ( $($ors) or* ));
        $crate::clause!(@expand $clause; $($tail)*);
    };
    (@expand $clause:expr; $name:expr => ( $($ors:tt) and* ) $(, $tail:tt)*) => {
        $crate::clause!(@to_string $clause, $name => ( $($ors) and* ));
        $crate::clause!(@expand $clause; $($tail)*);
    };
    (@expand $clause:expr; $name:expr => ( $($ors:tt) and* ), $($tail:tt)*) => {
        $crate::clause!(@to_string $clause, $name => ( $($ors) and* ));
        $crate::clause!(@expand $clause; $($tail)*);
    };
    (@expand $clause:expr; $name:expr => $value:expr, $($tail:tt)*) => {
        $crate::clause!(@expand $clause; $($tail)*);
        $crate::clause!(@to_string $clause, $name => $value);
    };
    (@expand $clause:expr; $name:expr => $value:expr $(, $tail:tt)*) => {
        $crate::clause!(@expand $clause; $($tail)*);
        $crate::clause!(@to_string $clause, $name => $value);
    };
    (@expand $clause:expr; $name:ident $(, $tail:tt)*) => {
        $crate::clause!(@expand $clause; $($tail, )*);
        if let Some(cla) = (&$name as &dyn $crate::Any).downcast_ref::<$crate::clause::Clause>() {
            if cla.valid {
                $clause.push(cla.to_string());
            }
        } else {
            $crate::clause!(@to_string $clause, stringify!($name) => $name);
        }
    };
    (@expand $clause:expr; $name:ident, $($tail:tt)*) => {
        $crate::clause!(@expand $clause; $($tail)*);
        if let Some(cla) = (&$name as &dyn $crate::Any).downcast_ref::<$crate::clause::Clause>() {
            if cla.valid {
                $clause.push(cla.to_string());
            }
        } else {
            $crate::clause!(@to_string $clause, stringify!($name) => $name);
        }
    };
    ($i:expr => $($tail:tt)*) => {
        {
            let mut clauses = $crate::clause::Clause::default();
            $crate::clause!(@expand (&mut clauses); $i => $($tail)*);
            clauses
        }
    };
    ($i:ident, $($tail:tt)*) => {
        {
            let mut clauses = $crate::clause::Clause::default();
            $crate::clause!(@expand (&mut clauses); $i, $($tail)*);
            clauses
        }
    };
    ($i:ident) => {
        {
            let mut clauses = $crate::clause::Clause::default();
            $crate::clause!(@expand (&mut clauses); $i);
            clauses
        }
    };
}

#[macro_export]
macro_rules! or_clause {
    () => { $crate::clause::Clause::default() };
    ($i:expr => $($tail:tt)*) => {
        {
            let mut clauses = $crate::clause::Clause::default();
            clauses.r#type = $crate::clause::ClauseType::Or;
            $crate::clause!(@expand (&mut clauses); $i => $($tail)*);
            clauses
        }
    };
    ($i:ident, $($tail:tt)*) => {
        {
            let mut clauses = $crate::clause::Clause::default();
            clauses.r#type = $crate::clause::ClauseType::Or;
            $crate::clause!(@expand (&mut clauses); $i, $($tail)*);
            clauses
        }
    };
    ($i:ident) => {
        {
            let mut clauses = $crate::clause::Clause::default();
            clauses.r#type = $crate::clause::ClauseType::Or;
            $crate::clause!(@expand (&mut clauses); $i);
            clauses
        }
    };
}
