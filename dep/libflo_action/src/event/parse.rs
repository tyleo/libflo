use AnyAction;
use error::*;
use libflo_std::{ Input, Libflo, Output };
use number_or_string::NumberOrString;
use std::mem;
use string;

pub unsafe fn parse(libflo: &Libflo, arg: (&NumberOrString, &str)) -> Result<Option<Box<AnyAction>>> {
    let event_mapper = libflo.get_event_mapper();
    let parse_event = event_mapper.get_by_module_name(string::module(), string::parse_event())?;

    let (number_or_string, _) = arg;
    let static_arg: (&'static NumberOrString, &'static str) = mem::transmute(arg);

    let results = parse_event
        .call(Input::Any(&static_arg))?
        .into_iter()
        .map(
            |result_output| {
                if let Output::Any(mut result_option_any) = result_output {
                    if let Some(result_option) = result_option_any.downcast_mut::<Option<Box<AnyAction>>>() {
                        Ok(mem::replace(result_option, None))
                    } else {
                        Err(ErrorKind::DowncastFailure("Option<Box<AnyAction>>".to_string()).into())
                    }
                } else {
                    Err(ErrorKind::ParserReturnedIncorrectType(number_or_string.clone()).into())
                }
            }
        );

    let mut errors = Vec::new();
    let mut oks = Vec::new();
    for result in results {
        match result {
            Ok(Some(item)) => oks.push(item),
            Ok(None) => { },
            Err(err) => errors.push(err),
        }
    }

    if errors.len() > 0 {
        Err(ErrorKind::ParserError(errors).into())
    } else if oks.len() > 1 {
        Err(ErrorKind::ParserReturnedMultipleActions(number_or_string.clone()).into())
    } else {
        Ok(oks.pop())
    }
}
