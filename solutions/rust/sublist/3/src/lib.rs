#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    let first_len = _first_list.len();
    let second_len = _second_list.len();

    if first_len == 0 && second_len > 0 {
        return Comparison::Sublist;
    }

    if second_len == 0 && first_len > 0 {
        return Comparison::Superlist;
    }

    if first_len < second_len {
        if _second_list.windows(first_len).any(|w| w == _first_list) {
            return Comparison::Sublist;
        }
    } else if _first_list.len() > _second_list.len() {
        if _first_list.windows(second_len).any(|w| w == _second_list) {
            return Comparison::Superlist;
        }
    }
    Comparison::Unequal
}