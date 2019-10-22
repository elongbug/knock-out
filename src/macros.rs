macro_rules! c_string {
    ( $string:tt ) => {
        concat!($string, "\0")
    };
}

macro_rules! to_ptr {
    ( $expr:expr ) => {
        $expr.as_ptr() as *const i8
    };
}

macro_rules! printk {
    ( $string:tt $(, $arg:expr )* ) => {
        printk( $string.as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_EMERG $string:tt $(, $arg:expr )* ) => {
        printk( concat!("0", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_ALERT $string:tt $(, $arg:expr )* ) => {
        printk( concat!("1", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_CRIT $string:tt $(, $arg:expr )* ) => {
        printk( concat!("2", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_ERR $string:tt $(, $arg:expr )* ) => {
        printk( concat!("3", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_WARNING $string:tt $(, $arg:expr )* ) => {
        printk( concat!("4", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_NOTICE $string:tt $(, $arg:expr )* ) => {
        printk( concat!("5", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_INFO $string:tt $(, $arg:expr )* ) => {
        printk( concat!("6", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_DEBUG $string:tt $(, $arg:expr )* ) => {
        printk( concat!("7", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_DEFAULT $string:tt $(, $arg:expr )* ) => {
        printk( concat!("d", $string).as_ptr() as *const i8, $( $arg ),* )
    };
    ( KERN_CONT $string:tt $(, $arg:expr )* ) => {
        printk( concat!("", $string).as_ptr() as *const i8, $( $arg ),* )
    };
}
