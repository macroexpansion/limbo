#[derive(Debug, Clone, PartialEq)]
pub enum AggFunc {
    Avg,
    Count,
    GroupConcat,
    Max,
    Min,
    StringAgg,
    Sum,
    Total,
}

impl AggFunc {
    pub fn to_string(&self) -> &str {
        match self {
            AggFunc::Avg => "avg",
            AggFunc::Count => "count",
            AggFunc::GroupConcat => "group_concat",
            AggFunc::Max => "max",
            AggFunc::Min => "min",
            AggFunc::StringAgg => "string_agg",
            AggFunc::Sum => "sum",
            AggFunc::Total => "total",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScalarFunc {
    Coalesce,
    Like,
    Abs,
    Upper,
    Lower,
    Random,
    Trim,
    LTrim,
    RTrim,
    Round,
    Length,
    Min,
    Max,
    Date,
    Time,
    Unicode,
}

impl std::fmt::Display for ScalarFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScalarFunc::Coalesce => write!(f, "coalesce"),
            ScalarFunc::Like => write!(f, "like(2)"),
            ScalarFunc::Abs => write!(f, "abs"),
            ScalarFunc::Upper => write!(f, "upper"),
            ScalarFunc::Lower => write!(f, "lower"),
            ScalarFunc::Random => write!(f, "random"),
            ScalarFunc::Trim => write!(f, "trim"),
            ScalarFunc::LTrim => write!(f, "ltrim"),
            ScalarFunc::RTrim => write!(f, "rtrim"),
            ScalarFunc::Round => write!(f, "round"),
            ScalarFunc::Length => write!(f, "length"),
            ScalarFunc::Min => write!(f, "min"),
            ScalarFunc::Max => write!(f, "max"),
            ScalarFunc::Date => write!(f, "date"),
            ScalarFunc::Time => write!(f, "time"),
            ScalarFunc::Unicode => write!(f, "unicode"),
        }
    }
}

#[derive(Debug)]
pub enum Func {
    Agg(AggFunc),
    Scalar(ScalarFunc),
}

impl Func {
    pub fn resolve_function(name: &str, arg_count: usize) -> Result<Func, ()> {
        match name {
            "avg" => Ok(Func::Agg(AggFunc::Avg)),
            "count" => Ok(Func::Agg(AggFunc::Count)),
            "group_concat" => Ok(Func::Agg(AggFunc::GroupConcat)),
            "max" if arg_count == 0 || arg_count == 1 => Ok(Func::Agg(AggFunc::Max)),
            "max" if arg_count > 1 => Ok(Func::Scalar(ScalarFunc::Max)),
            "min" if arg_count == 0 || arg_count == 1 => Ok(Func::Agg(AggFunc::Min)),
            "min" if arg_count > 1 => Ok(Func::Scalar(ScalarFunc::Min)),
            "string_agg" => Ok(Func::Agg(AggFunc::StringAgg)),
            "sum" => Ok(Func::Agg(AggFunc::Sum)),
            "total" => Ok(Func::Agg(AggFunc::Total)),
            "coalesce" => Ok(Func::Scalar(ScalarFunc::Coalesce)),
            "like" => Ok(Func::Scalar(ScalarFunc::Like)),
            "abs" => Ok(Func::Scalar(ScalarFunc::Abs)),
            "upper" => Ok(Func::Scalar(ScalarFunc::Upper)),
            "lower" => Ok(Func::Scalar(ScalarFunc::Lower)),
            "random" => Ok(Func::Scalar(ScalarFunc::Random)),
            "trim" => Ok(Func::Scalar(ScalarFunc::Trim)),
            "ltrim" => Ok(Func::Scalar(ScalarFunc::LTrim)),
            "rtrim" => Ok(Func::Scalar(ScalarFunc::RTrim)),
            "round" => Ok(Func::Scalar(ScalarFunc::Round)),
            "length" => Ok(Func::Scalar(ScalarFunc::Length)),
            "date" => Ok(Func::Scalar(ScalarFunc::Date)),
            "time" => Ok(Func::Scalar(ScalarFunc::Time)),
            "unicode" => Ok(Func::Scalar(ScalarFunc::Unicode)),
            _ => Err(()),
        }
    }
}
