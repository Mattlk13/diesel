use self::private::SqlOrdAggregate;
use crate::expression::functions::declare_sql_function;

#[declare_sql_function]
extern "SQL" {
    /// Represents a SQL `MAX` function. This function can only take types which are
    /// ordered.
    ///
    /// ## Aggregate Function Expression
    ///
    /// This function can be used as aggregate expression. See [`AggregateExpressionMethods`] for details.
    ///
    /// # Examples
    ///
    /// ## Normal function usage
    ///
    /// ```rust
    /// # include!("../../doctest_setup.rs");
    /// # use diesel::dsl::*;
    /// #
    /// # fn main() {
    /// #     use schema::animals::dsl::*;
    /// #     let connection = &mut establish_connection();
    /// assert_eq!(Ok(Some(8)), animals.select(max(legs)).first(connection));
    /// # }
    /// ```
    ///
    /// ## Aggregate function expression
    ///
    /// ```rust
    /// # include!("../../doctest_setup.rs");
    /// # use diesel::dsl::*;
    /// #
    /// # fn main() {
    /// #     use schema::animals::dsl::*;
    /// #     let connection = &mut establish_connection();
    /// #     #[cfg(not(feature = "mysql"))]
    /// assert_eq!(Ok(Some(4)), animals.select(max(legs).aggregate_filter(legs.lt(8))).first(connection));
    /// # }
    /// ```
    #[aggregate]
    fn max<ST: SqlOrdAggregate>(expr: ST) -> ST::Ret;

    /// Represents a SQL `MIN` function. This function can only take types which are
    /// ordered.
    ///
    /// ## Aggregate Function Expression
    ///
    /// This function can be used as aggregate expression. See [`AggregateExpressionMethods`] for details.
    ///
    /// # Examples
    ///
    /// ## Normal function usage
    ///
    /// ```rust
    /// # include!("../../doctest_setup.rs");
    /// # use diesel::dsl::*;
    /// #
    /// # fn main() {
    /// #     use schema::animals::dsl::*;
    /// #     let connection = &mut establish_connection();
    /// assert_eq!(Ok(Some(4)), animals.select(min(legs)).first(connection));
    /// # }
    /// ```
    ///
    /// ## Aggregate function expression
    ///
    /// ```rust
    /// # include!("../../doctest_setup.rs");
    /// # use diesel::dsl::*;
    /// #
    /// # fn main() {
    /// #     use schema::animals::dsl::*;
    /// #     let connection = &mut establish_connection();
    /// #     #[cfg(not(feature = "mysql"))]
    /// assert_eq!(Ok(Some(8)), animals.select(min(legs).aggregate_filter(legs.gt(4))).first(connection));
    /// # }
    /// ```
    #[aggregate]
    fn min<ST: SqlOrdAggregate>(expr: ST) -> ST::Ret;
}

mod private {
    use crate::sql_types::{IntoNullable, SingleValue, SqlOrd, SqlType};
    pub trait SqlOrdAggregate: SingleValue {
        type Ret: SqlType + SingleValue;
    }

    impl<T> SqlOrdAggregate for T
    where
        T: SqlOrd + IntoNullable + SingleValue,
        T::Nullable: SqlType + SingleValue,
    {
        type Ret = T::Nullable;
    }
}
