use error::*;
use libflo_action_std::{ BasicAction, dispatch, parse };
use libflo_std::{ impl_receive, LIBFLO, Result as FuncResult };
use serde_json;

#[no_mangle]
pub unsafe extern fn receive(arg: &str) -> FuncResult<()> {
    impl_receive(
        arg,
        |arg| -> Result<_> {
            let libflo = LIBFLO.read()?;

            let number_or_string = serde_json::from_str::<BasicAction>(arg)
                .map_err(|err| Error::from(ErrorKind::SerdeJsonError(err)))
                .chain_err(|| ErrorKind::ActionsDeserializationFailure(arg.to_string()))?
                .destructure();

            let parse_arg = (&number_or_string, arg);
            let parse_result = parse(&libflo, parse_arg)?;

            if let Some(action) = parse_result {
                Ok(dispatch(&libflo, action.as_ref())?)
            } else {
                Ok(())
            }
        }
    )
}
