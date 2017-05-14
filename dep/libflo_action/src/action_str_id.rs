use number_or_string::NumberOrString;

pub fn action_str_id<TStr0, TStr1>(module_name: TStr0, action_name: TStr1) -> NumberOrString
    where TStr0: AsRef<str>,
          TStr1: AsRef<str> {
        let result = format!("{} {}", module_name.as_ref(), action_name.as_ref());
        NumberOrString::String(result)
}
