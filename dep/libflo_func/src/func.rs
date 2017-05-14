register_funcs! {
    use serialization::ParameterSerde;
    use std::any;
    use std::string;

    types {
        module: func;
        serde: ParameterSerde;
    }

    parameters {
        None { serde: ParameterSerde::None },
        Any { input: any::Any, output: Box<any::Any>, serde: ParameterSerde::Any },
        String { input: str, output: string::String, serde: ParameterSerde::String }
    }
}
