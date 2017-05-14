use number_or_string::NumberOrString;

pub trait Action {
    fn get_type(&self) -> &NumberOrString;
}
