#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GroupFilter {
    Is { id: String, value: String },
    All(Vec<GroupFilter>),
    Any(Vec<GroupFilter>),
}

impl<T: ToString, U: ToString> From<(T, U)> for GroupFilter {
    fn from((id, value): (T, U)) -> Self {
        GroupFilter::Is {
            id: id.to_string(),
            value: value.to_string(),
        }
    }
}

impl GroupFilter {
    pub fn is(id: impl ToString, value: impl ToString) -> Self {
        GroupFilter::Is {
            id: id.to_string(),
            value: value.to_string(),
        }
    }

    pub fn both(left: impl Into<Self>, right: impl Into<Self>) -> Self {
        GroupFilter::All(vec![left.into(), right.into()])
    }

    pub fn three(left: impl Into<Self>, middle: impl Into<Self>, right: impl Into<Self>) -> Self {
        GroupFilter::All(vec![left.into(), middle.into(), right.into()])
    }

    pub fn either(left: impl Into<Self>, right: impl Into<Self>) -> Self {
        GroupFilter::Any(vec![left.into(), right.into()])
    }

    pub fn any<T: Into<GroupFilter>>(filters: Vec<T>) -> Self {
        GroupFilter::Any(filters.into_iter().map(|f| f.into()).collect())
    }

    pub fn any_values(id: impl ToString, values: Vec<impl ToString>) -> Self {
        GroupFilter::Any(
            values
                .into_iter()
                .map(|v| GroupFilter::Is {
                    id: id.to_string(),
                    value: v.to_string(),
                })
                .collect(),
        )
    }
}