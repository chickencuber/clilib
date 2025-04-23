#[macro_export]
macro_rules! helper {
    (exists_or_zero;) => {
        1
    };
    (exists_or_zero;$t:tt) => {
        $t
    };
    (exists;,$then:ty) => {
        $then
    };
    (exists;$t:tt, $then:ty) => {
        Vec<$then>
    };
    (exists_flag;,$then:ty) => {
        Option<$then>
    };
    (exists_flag;$t:tt, $then:ty) => {
        Vec<$then>
    };
    (exists_add;,$var:ident, $type:ty, $d:ident, $db:ident) => {
        $var = <$type>::from($d[$db].clone());
        $d.remove($db);
    };
    (exists_add;$t:tt,$var:ident, $type:ty, $d:ident, $db:ident) => {
        for _ in 0..$t {
            $var.push(<$type>::from($d[$db].clone()));
            $d.remove($db);
        }
    };
    (exists_add_flag;,$var:ident, $type:ty, $d:ident, $db:expr) => {
        $var = Some(<$type>::from($d[$db].clone()));
        $d.remove($db);
    };
    (exists_add_flag;$t:tt,$var:ident, $type:ty, $d:ident, $db:expr) => {
        for _ in 0..$t {
            $var.push(<$type>::from($d[$db].clone()));
            $d.remove($db);
        }
    };
    (exists_declare;,$var:ident, $type:ty) => {
        let $var: $type;
    };
    (exists_declare;$t:tt,$var:ident, $type:ty) => {
        let mut $var: Vec<$type> = Vec::new();
    };
    (exists_declare_flag;,$var:ident, $type:ty) => {
        let mut $var: Option<$type> = None;
    };
    (exists_declare_flag;$t:tt,$var:ident, $type:ty) => {
        let mut $var: Vec<$type> = Vec::new();
    };
}


#[macro_export]
macro_rules! define {
    ($name:ident; help: $help:literal; flags {
        $($fname:ident: $ftype:ty = $($flag:literal)|* $(=> [$fnum:literal])?),*$(,)?
    }; args {
        $($aname:ident: $atype:ty $(=> [$num:literal])?),*$(,)? 
    };
    $(rest => $rname:ident: $rtype:ty;)?) => {
        #[derive(Debug)]
        struct $name {
            $($fname: tt_call::tt_if!{
                condition = [{tt_equal::tt_equal}]
                    input = [{ $ftype bool }]
                    true = [{
                        bool
                    }]
                false = [{
                    $crate::helper!(exists_flag;$($fnum)?, $ftype) 
                }]
            },)*
            $(
                $aname: $crate::helper!(exists;$($num)?, $atype)
            ),*,
            $(
                $rname: Vec<$rtype>
            )?
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
                            $crate::helper!(exists_declare_flag;$($fnum)?, $fname, $ftype);
                        }]
                    }
                )*
                $(
                    $crate::helper!(exists_declare;$($num)?,$aname, $atype);
                )*
                $(
                    let mut $rname: Vec<$rtype> = Vec::new();
                )?
                __args.remove(0);
                if __args.contains(&"-help".to_string()) || __args.contains(&"--help".to_string()) {
                    panic!("{}", $help);
                }
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
                        if v.0 {
                            __i += v.1;
                            if __args[__i].starts_with("-") {
                                panic!("the flag requires an argument");
                            }
                        }
                        __i += 1;
                    }

                    $crate::helper!(exists_add; $($num)?, $aname, $atype, __args, __i);
                    )*
                        $(
                            while __args.len() - __i > 0 {
                                if __args[__i].starts_with("-") {
                                    if $rname.len() > 0 {
                                        break;
                                    }
                                    while __args[__i].starts_with("-") {
                                        __args[__i].remove(0);
                                        if __args[__i].starts_with("-") {
                                            __args[__i].remove(0);
                                        } 
                                        let v = Self::has_args(__args[__i].clone());
                                        __args[__i].insert(0, '-');
                                        if v.0 {
                                            __i += v.1;
                                            if __args[__i].starts_with("-") {
                                                panic!("the flag requires an argument");
                                            }
                                        }
                                        __i += 1;
                                    }
                                }
                                $rname.push(<$rtype>::from(__args[__i].clone()));
                                __args.remove(__i);

                            }
                    )?
                        while __args.len() > 0 {
                            let mut __ch = __args[0].clone();
                            if !__ch.starts_with("-") {
                                panic!("too many arguments");
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
                                            $crate::helper!(exists_add_flag;$($fnum)?, $fname, $ftype, __args, 1);
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
                    $($rname)?
                }
            }
            fn has_args(v: String) -> (bool, usize) {
                $(
                    if $(v == $flag)||* {
                        tt_call::tt_if!{
                            condition = [{tt_equal::tt_equal}]
                                input = [{ $ftype bool }]
                                true = [{
                                    return (false, 0);
                                }]
                            false = [{
                                return (true, $crate::helper!(exists_or_zero;$($fnum)?));
                            }]
                        }   
                    }
                )*
                    panic!("invalid flag {}", v);
            }
        }
    };
}
