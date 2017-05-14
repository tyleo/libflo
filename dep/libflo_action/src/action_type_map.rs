use { ActionMapper, ExtActionMapper };
use error::*;
use libflo_std::{ ModuleMapper };
use number_or_string::NumberOrString;
use std::collections::HashMap;

pub struct ActionTypeMap<T> {
    raw_map: HashMap<usize, T>,
}

impl <T> ActionTypeMap<T> {
    pub fn new<'a, 'b, TIter>(module_mapper: &ModuleMapper, action_mapper: &ActionMapper, iter: TIter) -> Result<Self>
        where TIter: IntoIterator<Item = (&'a str, &'b str, T)> {
        let ext_action_mapper = ExtActionMapper::new(action_mapper, module_mapper);

        let mut raw_map = HashMap::new();
        for (module, action, function) in iter {
            let action_id = ext_action_mapper.get_by_module_name(module, action)?;
            if let Some(_) = raw_map.insert(action_id, function) {
                return Err(ErrorKind::TypeAlreadyMapped(module.to_string(), action.to_string()).into());
            }
        }

        Ok(ActionTypeMap { raw_map: raw_map })
    }

    pub fn get(&self, action_type: &NumberOrString, module_mapper: &ModuleMapper, action_mapper: &ActionMapper) -> Result<Option<&T>> {
        let action_id = match action_type {
            &NumberOrString::Number(value) => value as usize,
            &NumberOrString::String(ref value) => {
                let mut str_iter = value.split_whitespace().into_iter();
                if let Some(module) = str_iter.next() {
                    if let Some(event) = str_iter.next() {
                        if let Some(extra) = str_iter.next() {
                            let mut total_words = format!("{} {} {}", module, event, extra);
                            for current_word in str_iter {
                                total_words = format!("{} {}", total_words, current_word);
                            }
                            return Err(Error::from(ErrorKind::TypeContainsMoreThanTwoWords(total_words)));
                        } else {
                            let ext_action_mapper = ExtActionMapper::new(action_mapper, module_mapper);
                            ext_action_mapper.get_by_module_name(module, event)?
                        }
                    } else {
                        return Err(Error::from(ErrorKind::TypeOnlyContainsOneWord(module.to_string())));
                    }
                } else {
                    return Err(Error::from(ErrorKind::TypeIsEmpty));
                }
            }
        };

        Ok(self.raw_map.get(&action_id))
    }
}
