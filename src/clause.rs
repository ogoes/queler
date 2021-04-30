use crate::value;

#[derive(Default, Clone)]
pub struct Clause {
    and: String,
    or: String,
    pub valid: bool,
}

impl From<Clause> for value::Value {
    fn from(v: Clause) -> Self {
        value::Value::String(format!("({})", v.to_string()))
    }
}

impl std::fmt::Display for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = if self.and.len() > 0usize {
            &self.and
        } else {
            &self.or
        };
        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Clause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clause: {}", self)
    }
}

impl Clause {
    pub fn _and_push(&mut self, clause: String) {
        self.valid = true;
        self.and.push_str(
            format!(
                "{}{}",
                if self.and.len() > 0usize { " AND " } else { "" },
                clause
            )
            .as_str(),
        );
    }

    pub fn _or_push(&mut self, clause: String) {
        self.valid = true;
        self.or.push_str(
            format!(
                "{}{}",
                if self.and.len() > 0usize { " OR " } else { "" },
                clause
            )
            .as_str(),
        );
    }
}

#[macro_export]
macro_rules! clause {
    () => { $crate::clause::Clause::default() };
    (@to_string $value:expr) => (
        format!("{}", $crate::value::Value::from($value))
    );
    (@to_string $clause:expr, $name:expr => ( $($ors:tt) or* )) => (

        let mut vec = vec![];
        $(
            vec.push($crate::clause!(@to_string $ors));
        )*

        let mut or_clause = "(".to_string();
        for (index, or) in vec.iter().enumerate() {
            let value = match value {
                $crate::value::Value::NULL => format!("{} IS NULL", $name),
                $crate::value::Value::Boolean(_) => format!("CAST({} AS INTEGER) = {}", $name, or),
                _ => format!("{} = {}", $name, or)
            };
            or_clause.push_str(value.as_str());

            if index < vec.len() - 1 {
                or_clause.push_str(format!(" OR ").as_str())
            }
        }

        or_clause.push_str(")");

        $clause._and_push(or_clause);
    );
    (@to_string $clause:expr, $name:expr => $value:expr) => {

        let value = $crate::value::Value::from($value);

        let value = match value {
            $crate::value::Value::NULL => format!("{} IS NULL", $name),
            $crate::value::Value::Boolean(v) => format!("CAST({} AS INTEGER) = {}", $name, value),
            _ => format!("{} = {}", $name, value)
        };

        $clause._and_push(value);
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
                $clause._and_push(cla.to_string());
            }
        } else {
            $crate::clause!(@to_string $clause, stringify!($name) => $name);
        }
    };
    (@expand $clause:expr; $name:ident, $($tail:tt)*) => {
        $crate::clause!(@expand $clause; $($tail)*);
        if let Some(cla) = (&$name as &dyn $crate::Any).downcast_ref::<$crate::clause::Clause>() {
            if cla.valid {
                $clause._and_push(cla.to_string());
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
    (@to_string $value:expr) => (
        format!("{}", $crate::value::Value::from($value))
    );
    (@to_string $clause:expr, $name:expr => ( $($ors:tt) and* )) => (

        let mut vec = vec![];
        $(
            vec.push($crate::clause!(@to_string $ors));
        )*

        let mut or_clause = "(".to_string();
        for (index, or) in vec.iter().enumerate() {
            let value = match value {
                $crate::value::Value::NULL => format!("{} IS NULL", $name),
                $crate::value::Value::Boolean(_) => format!("CAST({} AS INTEGER) = {}", $name, or),
                _ => format!("{} = {}", $name, or)
            };
            or_clause.push_str(value.as_str());

            if index < vec.len() - 1 {
                or_clause.push_str(format!(" AND ").as_str())
            }
        }

        or_clause.push_str(")");

        $clause._or_push(or_clause);
    );
    (@to_string $clause:expr, $name:expr => $value:expr) => {

        let value = $crate::value::Value::from($value);

        let value = match value {
            $crate::value::Value::NULL => format!("{} IS NULL", $name),
            $crate::value::Value::Boolean(v) => format!("CAST({} AS INTEGER) = {}", $name, value),
            _ => format!("{} = {}", $name, value)
        };

        $clause._or_push(value);
    };
    (@expand $clause:expr;) => {};
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
        $crate::clause!(@expand $clause; $($tail)*);
        if let Some(cla) = (&$name as &dyn $crate::Any).downcast_ref::<$crate::clause::Clause>() {
            if cla.valid {
                $clause._or_push(cla.to_string());
            }
        } else {
            $crate::clause!(@to_string $clause, stringify!($name) => $name);
        }
    };
    (@expand $clause:expr; $name:ident, $($tail:tt)*) => {
        $crate::clause!(@expand $clause; $($tail)*);
        if let Some(cla) = (&$name as &dyn $crate::Any).downcast_ref::<$crate::clause::Clause>() {
            if cla.valid {
                $clause._or_push(cla.to_string());
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
