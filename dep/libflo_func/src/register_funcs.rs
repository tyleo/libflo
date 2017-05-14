macro_rules! register_impl {
    (
        @0

        $(
            use $used_path: path;
        )* @

        $module_path: path,
        $serde_ty: ty,
        $none_ident: ident,
        $none_serde_expr: expr,

        inputs {
            $(
                $input_ident: ident {
                    $input_ty: ty,
                    $input_serde_expr: expr
                }
            ),*
        },

        outputs {
            $(
                $output_ident: ident {
                    $output_ty: ty,
                    $output_serde_expr: expr
                }
            ),*
        }
    ) => {
        register_impl!(
            @1

            $(
                use $used_path;
            )* @

            $module_path,
            $none_ident,

            outputs {
                $(
                    $output_ident { $output_ty }
                ),*
            }
        );

        register_impl!(
            @3

            $(
                use $used_path;
            )* @

            $module_path,
            $none_ident,

            inputs {
                $(
                    $input_ident { $input_ty }
                ),*
            }

            outputs {
                $(
                    $output_ident { $output_ty }
                ),*
            }
        );

        register_impl!(
            @5

            $module_path,
            $serde_ty,
            $none_ident,
            $none_serde_expr,

            inputs {
                $(
                    $input_ident {
                        $input_ty,
                        $input_serde_expr
                    }
                ),*
            },

            outputs {
                $(
                    $output_ident {
                        $output_ty,
                        $output_serde_expr
                    }
                ),*
            }
        );
    };

    (
        @1

        $(
            use $used_path: path;
        )* @

        $module_path: path,
        $none_ident: ident,

        outputs {
            $(
                $output_ident: ident { $output_ty: ty }
            ),*
        }
    ) => {
        #[allow(non_snake_case)]
        mod $none_ident {
            #[allow(non_snake_case)]
            pub mod $none_ident {
                use error::*;
                use $module_path::{ Func as FuncTrait, Input, Output, Parameter };
                use libflo_error::Result as FuncResult;
                use sharedlib::{ FuncArc, LibArc, Symbol };

                #[derive(Clone)]
                pub struct Func {
                    func: FuncArc<fn() -> FuncResult<()>>,
                }

                impl Func {
                    pub unsafe fn new<TStr>(lib: &LibArc, symbol: TStr) -> Result<Self>
                        where TStr: AsRef<str> {
                        let symbol_ref = symbol.as_ref();
                        let func = lib
                            .find_func(symbol_ref)
                            .chain_err(|| ErrorKind::SymbolCouldNotLoad(symbol_ref.to_string()))?;

                        Ok(Func { func: func })
                    }
                }

                impl FuncTrait for Func {
                    fn get_type(&self) -> (Parameter, Parameter) {
                        (Parameter::$none_ident, Parameter::$none_ident)
                    }

                    unsafe fn call(&self, input: Input) -> Result<Output> {
                        match input {
                            Input::$none_ident => {
                                let func = self.func.get();
                                func()?;
                                Ok(Output::$none_ident)
                            },
                            given => {
                                let required = self.get_type();
                                let given = Parameter::from(&given);
                                Err(ErrorKind::FuncParameterIncorrect(required.0, given).into())
                            }
                        }
                    }

                    fn clone_into_box(&self) -> Box<FuncTrait> {
                        Box::new(self.clone())
                    }
                }
            }

            register_impl!(
                @2

                $(
                    use $used_path;
                )* @

                $module_path,
                $none_ident,

                outputs {
                    $(
                        $output_ident { $output_ty }
                    ),*
                }
            );
        }
    };

    (
        @2

        $(
            use $used_path: path;
        )* @

        $module_path: path,
        $none_ident: ident,

        outputs {
            $output_ident: ident { $output_ty: ty },
            $(
                $next_output_ident: ident { $next_output_ty: ty }
            ),*
        }
    ) => {
        register_impl!(
            @2

            $(
                use $used_path;
            )* @

            $module_path,
            $none_ident,

            outputs {
                $output_ident { $output_ty }
            }
        );

        register_impl!(
            @2

            $(
                use $used_path;
            )* @

            $module_path,
            $none_ident,

            outputs {
                $(
                    $next_output_ident { $next_output_ty }
                ),*
            }
        );
    };

    (
        @2

        $(
            use $used_path: path;
        )* @

        $module_path: path,
        $none_ident: ident,

        outputs {
            $output_ident: ident { $output_ty: ty }
        }
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused_imports)]
        pub mod $output_ident {
            $(
                use $used_path;
            )*

            use error::*;
            use $module_path::{ Func as FuncTrait, Input, Output, Parameter };
            use libflo_error::Result as FuncResult;
            use sharedlib::{ FuncArc, LibArc, Symbol };

            #[derive(Clone)]
            pub struct Func {
                func: FuncArc<fn() -> FuncResult<$output_ty>>,
            }

            impl Func {
                pub unsafe fn new<TStr>(lib: &LibArc, symbol: TStr) -> Result<Self>
                    where TStr: AsRef<str> {
                    let symbol_ref = symbol.as_ref();
                    let func = lib
                        .find_func(symbol_ref)
                        .chain_err(|| ErrorKind::SymbolCouldNotLoad(symbol_ref.to_string()))?;

                    Ok(Func { func: func })
                }
            }

            impl FuncTrait for Func {
                fn get_type(&self) -> (Parameter, Parameter) {
                    (Parameter::$none_ident, Parameter::$output_ident)
                }

                unsafe fn call(&self, input: Input) -> Result<Output> {
                    match input {
                        Input::$none_ident => {
                            let func = self.func.get();
                            let result = func()?;
                            Ok(Output::$output_ident(result))
                        },
                        given => {
                            let required = self.get_type();
                            let given = Parameter::from(&given);
                            Err(ErrorKind::FuncParameterIncorrect(required.0, given).into())
                        }
                    }
                }

                fn clone_into_box(&self) -> Box<FuncTrait> {
                    Box::new(self.clone())
                }
            }
        }
    };

    (
        @3

        $(
            use $used_path: path;
        )* @

        $module_path: path,
        $none_ident: ident,

        inputs {
            $input_ident: ident { $input_ty: ty },
            $(
                $next_input_ident: ident { $next_input_ty: ty }
            ),*
        }

        outputs {
            $(
                $output_ident: ident { $output_ty: ty }
            ),*
        }
    ) => {
        register_impl!(
            @3

            $(
                use $used_path;
            )* @

            $module_path,
            $none_ident,

            inputs {
                $input_ident { $input_ty }
            }

            outputs {
                $(
                    $output_ident { $output_ty }
                ),*
            }
        );

        register_impl!(
            @3

            $(
                use $used_path;
            )* @

            $module_path,
            $none_ident,

            inputs {
                $(
                    $next_input_ident { $next_input_ty }
                ),*
            }

            outputs {
                $(
                    $output_ident { $output_ty }
                ),*
            }
        );
    };

    (
        @3

        $(
            use $used_path: path;
        )* @

        $module_path: path,
        $none_ident: ident,

        inputs {
            $input_ident: ident { $input_ty: ty }
        }

        outputs {
            $(
                $output_ident: ident { $output_ty: ty }
            ),*
        }
    ) => {
        #[allow(non_snake_case)]
        mod $input_ident {
            #[allow(non_snake_case)]
            #[allow(unused_imports)]
            pub mod $none_ident {
                $(
                    use $used_path;
                )*

                use error::*;
                use $module_path::{ Func as FuncTrait, Input, Output, Parameter };
                use libflo_error::Result as FuncResult;
                use sharedlib::{ FuncArc, LibArc, Symbol };

                #[derive(Clone)]
                pub struct Func {
                    func: FuncArc<fn(&$input_ty) -> FuncResult<()>>,
                }

                impl Func {
                    pub unsafe fn new<TStr>(lib: &LibArc, symbol: TStr) -> Result<Self>
                        where TStr: AsRef<str> {
                        let symbol_ref = symbol.as_ref();
                        let func = lib
                            .find_func(symbol_ref)
                            .chain_err(|| ErrorKind::SymbolCouldNotLoad(symbol_ref.to_string()))?;

                        Ok(Func { func: func })
                    }
                }

                impl FuncTrait for Func {
                    fn get_type(&self) -> (Parameter, Parameter) {
                        (Parameter::$input_ident, Parameter::$none_ident)
                    }

                    unsafe fn call(&self, input: Input) -> Result<Output> {
                        match input {
                            Input::$input_ident(input) => {
                                let func = self.func.get();
                                func(input)?;
                                Ok(Output::$none_ident)
                            },
                            given => {
                                let required = self.get_type();
                                let given = Parameter::from(&given);
                                Err(ErrorKind::FuncParameterIncorrect(required.0, given).into())
                            }
                        }
                    }

                    fn clone_into_box(&self) -> Box<FuncTrait> {
                        Box::new(self.clone())
                    }
                }
            }

            register_impl!(
                @4

                $(
                    use $used_path;
                )* @

                $module_path,
                $none_ident,

                inputs {
                    $input_ident { $input_ty }
                }

                outputs {
                    $(
                        $output_ident { $output_ty }
                    ),*
                }
            );
        }
    };

    (
        @4

        $(
            use $used_path: path;
        )* @

        $module_path: path,
        $none_ident: ident,

        inputs {
            $input_ident: ident { $input_ty: ty }
        }

        outputs {
            $output_ident: ident { $output_ty: ty },
            $(
                $next_output_ident: ident { $next_output_ty: ty }
            ),*
        }
    ) => {
        register_impl!(
            @4

            $(
                use $used_path;
            )* @

            $module_path,
            $none_ident,

            inputs {
                $input_ident { $input_ty }
            }

            outputs {
                $output_ident { $output_ty }
            }
        );

        register_impl!(
            @4

            $(
                use $used_path;
            )* @

            $module_path,
            $none_ident,

            inputs {
                $input_ident { $input_ty }
            }

            outputs {
                $(
                    $next_output_ident { $next_output_ty }
                ),*
            }
        );
    };

    (
        @4

        $(
            use $used_path: path;
        )* @

        $module_path: path,
        $none_ident: ident,

        inputs {
            $input_ident: ident { $input_ty: ty }
        }

        outputs {
            $output_ident: ident { $output_ty: ty }
        }
    ) => {
        #[allow(non_snake_case)]
        #[allow(unused_imports)]
        pub mod $output_ident {
            $(
                use $used_path;
            )*

            use error::*;
            use $module_path::{ Func as FuncTrait, Input, Output, Parameter };
            use libflo_error::Result as FuncResult;
            use sharedlib::{ FuncArc, LibArc, Symbol };

            #[derive(Clone)]
            pub struct Func {
                func: FuncArc<fn(&$input_ty) -> FuncResult<$output_ty>>,
            }

            impl Func {
                pub unsafe fn new<TStr>(lib: &LibArc, symbol: TStr) -> Result<Self>
                    where TStr: AsRef<str> {
                    let symbol_ref = symbol.as_ref();
                    let func = lib
                        .find_func(symbol_ref)
                        .chain_err(|| ErrorKind::SymbolCouldNotLoad(symbol_ref.to_string()))?;

                    Ok(Func { func: func })
                }
            }

            impl FuncTrait for Func {
                fn get_type(&self) -> (Parameter, Parameter) {
                    (Parameter::$input_ident, Parameter::$output_ident)
                }

                unsafe fn call(&self, input: Input) -> Result<Output> {
                    match input {
                        Input::$input_ident(input) => {
                            let func = self.func.get();
                            let result = func(input)?;
                            Ok(Output::$output_ident(result))
                        },
                        given => {
                            let required = self.get_type();
                            let given = Parameter::from(&given);
                            Err(ErrorKind::FuncParameterIncorrect(required.0, given).into())
                        }
                    }
                }

                fn clone_into_box(&self) -> Box<FuncTrait> {
                    Box::new(self.clone())
                }
            }
        }
    };

    (
        @5

        $module_path: path,
        $serde_ty: ty,
        $none_ident: ident,
        $none_serde_expr: expr,

        inputs {
            $(
                $input_ident: ident {
                    $input_ty: ty,
                    $input_serde_expr: expr
                }
            ),*
        },

        outputs {
            $(
                $output_ident: ident {
                    $output_ty: ty,
                    $output_serde_expr: expr
                }
            ),*
        }
    ) => {
        pub unsafe fn new_func<TStr>(lib: &LibArc, symbol: TStr, input_type: $serde_ty, output_type: $serde_ty) -> Result<Box<Func>>
            where TStr: AsRef<str> {
            if input_type == $none_serde_expr {
                if output_type == $none_serde_expr {
                    use self::$none_ident::{ $none_ident as CurrentModule };
                    return Ok(Box::new(CurrentModule::Func::new(lib, symbol)?));
                } $( else if output_type == $output_serde_expr {
                    use self::$none_ident::{ $output_ident as CurrentModule };
                    return Ok(Box::new(CurrentModule::Func::new(lib, symbol)?));
                } )* else {
                    return Err(ErrorKind::FuncTypeCouldNotBeMatched((Box::new(input_type), Box::new(output_type))).into());
                }
            }

            register_impl!(
                @6

                $module_path,
                $serde_ty,
                $none_ident,
                $none_serde_expr,

                inputs {
                    $(
                        $input_ident {
                            $input_ty,
                            $input_serde_expr
                        }
                    ),*
                },

                outputs {
                    $(
                        $output_ident {
                            $output_ty,
                            $output_serde_expr
                        }
                    ),*
                },

                lib,
                symbol,
                input_type,
                output_type
            );

            Err(ErrorKind::FuncTypeCouldNotBeMatched((Box::new(input_type), Box::new(output_type))).into())
        }
    };

    (
        @6

        $module_path: path,
        $serde_ty: ty,
        $none_ident: ident,
        $none_serde_expr: expr,

        inputs {
            $input_ident: ident {
                $input_ty: ty,
                $input_serde_expr: expr
            },
            $(
                $next_input_ident: ident {
                    $next_input_ty: ty,
                    $next_input_serde_expr: expr
                }
            ),*
        },

        outputs {
            $(
                $output_ident: ident {
                    $output_ty: ty,
                    $output_serde_expr: expr
                }
            ),*
        },

        $lib: expr,
        $symbol: expr,
        $input_type: expr,
        $output_type: expr
    ) => {
        register_impl!(
            @6

            $module_path,
            $serde_ty,
            $none_ident,
            $none_serde_expr,

            inputs {
                $input_ident {
                    $input_ty,
                    $input_serde_expr
                }
            },

            outputs {
                $(
                    $output_ident {
                        $output_ty,
                        $output_serde_expr
                    }
                ),*
            },

            $lib,
            $symbol,
            $input_type,
            $output_type
        );

        register_impl!(
            @6

            $module_path,
            $serde_ty,
            $none_ident,
            $none_serde_expr,

            inputs {
                $(
                    $next_input_ident {
                        $next_input_ty,
                        $next_input_serde_expr
                    }
                ),*
            },

            outputs {
                $(
                    $output_ident {
                        $output_ty,
                        $output_serde_expr
                    }
                ),*
            },

            $lib,
            $symbol,
            $input_type,
            $output_type
        );
    };

    (
        @6

        $module_path: path,
        $serde_ty: ty,
        $none_ident: ident,
        $none_serde_expr: expr,

        inputs {
            $input_ident: ident {
                $input_ty: ty,
                $input_serde_expr: expr
            }
        },

        outputs {
            $(
                $output_ident: ident {
                    $output_ty: ty,
                    $output_serde_expr: expr
                }
            ),*
        },

        $lib: expr,
        $symbol: expr,
        $input_type: expr,
        $output_type: expr
    ) => {
        if $input_type == $input_serde_expr {
            if $output_type == $none_serde_expr {
                use self::$input_ident::{ $none_ident as CurrentModule };
                return Ok(Box::new(CurrentModule::Func::new($lib, $symbol)?));
            } $( else if $output_type == $output_serde_expr {
                use self::$input_ident::{ $output_ident as CurrentModule };
                return Ok(Box::new(CurrentModule::Func::new($lib, $symbol)?));
            } )* else {
                return Err(ErrorKind::FuncTypeCouldNotBeMatched((Box::new($input_type), Box::new($output_type))).into());
            }
        }
    };
}

macro_rules! register_funcs {
    {
        $(
            use $used_path: path;
        )*

        types {
            module: $module_path: path;
            serde: $serde_ty: ty;
        }

        parameters {
            $none_ident: ident { serde: $none_serde_expr: expr },
            $(
                $current_ident: ident {
                    input: $input_ty: ty, output: $output_ty: ty, serde: $current_serde_expr: expr
                }
            ),*
        }
    } => {
        $(
            use $used_path;
        )*

        use error::*;
        use sharedlib::LibArc;

        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Parameter {
            $none_ident,
            $(
                $current_ident
            ),*
        }

        #[derive(Clone, Debug)]
        pub enum Input<'a> {
            $none_ident,
            $(
                $current_ident(&'a $input_ty)
            ),*
        }

        #[derive(Debug)]
        pub enum Output {
            $none_ident,
            $(
                $current_ident($output_ty)
            ),*
        }

        pub trait Func: Send + Sync {
            fn get_type(&self) -> (Parameter, Parameter);

            unsafe fn call(&self, input: Input) -> Result<Output>;

            fn clone_into_box(&self) -> Box<Func>;
        }

        impl <'a, 'b> From<&'a Input<'b>> for Parameter {
            fn from(value: &Input) -> Parameter {
                match value {
                    &Input::$none_ident => Parameter::$none_ident,
                    $(
                        &Input::$current_ident(_) => Parameter::$current_ident
                    ),*
                }
            }
        }

        impl <'a> From<&'a Output> for Parameter {
            fn from(value: &Output) -> Parameter {
                match value {
                    &Output::$none_ident => Parameter::$none_ident,
                    $(
                        &Output::$current_ident(_) => Parameter::$current_ident
                    ),*
                }
            }
        }

        impl <'a> From<&'a $serde_ty> for Result<Parameter> {
            fn from(value: &$serde_ty) -> Result<Parameter> {
                if *value == $none_serde_expr {
                    Ok(Parameter::$none_ident)
                } $( else if *value == $current_serde_expr {
                    Ok(Parameter::$current_ident)
                })* else {
                    panic!();
                }
            }
        }

        register_impl!(
            @0

            $(
                use $used_path;
            )* @

            $module_path,
            $serde_ty,
            $none_ident,
            $none_serde_expr,

            inputs {
                $(
                    $current_ident {
                        $input_ty,
                        $current_serde_expr
                    }
                ),*
            },

            outputs {
                $(
                    $current_ident {
                        $output_ty,
                        $current_serde_expr
                    }
                ),*
            }
        );
    }
}
