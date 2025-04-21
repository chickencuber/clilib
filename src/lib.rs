#[macro_export]
macro_rules! define {
    ($name:ident; flags {
        $($fname:ident: $ftype:ty = $($flag:literal)|*),*$(,)?
    }; args {
        $($aname:ident: $atype:ty),*$(,)? 
    };) => {
        #[derive(Debug)]
        struct $name {
            $($fname: tt_call::tt_if!{
                condition = [{tt_equal::tt_equal}]
                    input = [{ $ftype bool }]
                    true = [{
                        bool
                    }]
                false = [{
                    Option<$ftype> 
                }]
            },)*
            $($aname: $atype,)*
        } 
        impl $name {
            pub fn from(mut __args: Vec<String>) -> Self{
                $(
                    tt_call::tt_if!{
                        condition = [{tt_equal::tt_equal}]
                        input = [{ $ftype bool }]
                        true = [{
                            let mut $fname: bool = false;
                        }]
                        false = [{
                            let mut $fname: Option<$ftype> = None;
                        }]
                    }
                )*
                $(
                    let $aname: $atype;
                )*
                __args.remove(0);
                let mut __i = 0;
                $(
                    if __args.len() == 0 {
                        panic!("missing argument: '{}'", stringify!($aname));
                    }
                    while __args[__i].starts_with("-") {
                        __args[__i].remove(0);
                        if __args[__i].starts_with("-") {
                            __args[__i].remove(0);
                        } 
                        let v = Self::has_args(__args[__i].clone());
                        __args[__i].insert(0, '-');
                        if v {
                            __i += 1;
                            if __args[__i].starts_with("-") {
                                panic!("the flag requires an argument");
                            }
                        }
                        __i += 1;
                    }
                    $aname = <$atype>::from(__args[__i].clone());
                    __args.remove(__i);
                )*
                while __args.len() > 0 {
                    let mut __ch = __args[0].clone();
                    if !__ch.starts_with("-") {
                        panic!("unexpected error");
                    }
                    __ch.remove(0);
                    $(
                        if $(__ch == $flag)||* {
                            tt_call::tt_if!{
                                condition = [{tt_equal::tt_equal}]
                                    input = [{ $ftype bool }]
                                    true = [{
                                        $fname = true;
                                    }]
                                false = [{
                                    if __args[1].starts_with("-") {
                                        panic!("flags requires an argument");
                                    }
                                    $fname = Some(<$ftype>::from(__args[1].clone()));
                                    __args.remove(0);
                                }]
                            }   
                            __args.remove(0);
                            continue;
                        }
                    )*
                    panic!("invalid flag {}", __ch);
                }
                return Self {
                    $($fname,)*
                    $($aname,)*
                }
            }
            fn has_args(v: String) -> bool {
                $(
                    if $(v == $flag)||* {
                        tt_call::tt_if!{
                            condition = [{tt_equal::tt_equal}]
                                input = [{ $ftype bool }]
                                true = [{
                                    return false;
                                }]
                            false = [{
                                return true;
                            }]
                        }   
                    }
                )*
                    panic!("invalid flag {}", v);
            }
        }
    };
}
