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
    } else if _first_list.len() < _second_list.len() {
        if _first_list.is_empty() || _second_list.windows(_first_list.len()).any(|w| w == _first_list) {
            return Comparison::Sublist;
        }
    } else if _second_list.is_empty() || _first_list.windows(_second_list.len()).any(|w| w == _second_list) {
        return Comparison::Superlist;

    }    
    Comparison::Unequal
}
