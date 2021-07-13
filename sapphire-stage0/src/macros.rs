#[macro_export]
macro_rules! assert_parse{
    ($left: expr, $right: expr) => {
        let res: nom::IResult<_, _, (_, ErrorKind)> = $left;
        assert_eq!(res, $right);
    };
    ($left: expr, $( $pattern:pat )|+ $( if $guard: expr )? $(,)?) => {
        let res: nom::IResult<_, _, (_, ErrorKind)> = $left;
        assert!(matches!(res, $($pattern)|+ $( if $guard )?));
    };
}
