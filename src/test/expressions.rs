use super::*;

macro_rules! test_expressions {
    (! $name:ident : $exp:literal ; $( $tail:tt )*) => {
        #[test]
        fn $name() -> Result<()> {
            let tmpl = Templar::global().parse_expression($exp)?;
            let context = StandardContext::new();
            let result = tmpl.exec(&context);
            assert!(result.is_err());
            Ok(())
        }
        test_expressions! {
            $( $tail )*
        }
    };
    ($name:ident : $exp:literal == $res:expr ; $( $tail:tt )*) => {
        #[test]
        fn $name() -> Result<()> {
            let tmpl = Templar::global().parse_expression($exp)?;
            let context = StandardContext::new();
            let result = tmpl.exec(&context)?;
            let cmp: Document = ($res).into();
            assert_eq!(result, cmp, "{} expression '{}' result -> {:?}", stringify!($name), $exp, result);
            Ok(())
        }
        test_expressions! {
            $( $tail )*
        }
    };
    ($name:ident : $exp:literal != $res:expr ; $( $tail:tt )*) => {
        #[test]
        fn $name() -> Result<()> {
            let tmpl = Templar::global().parse_expression($exp)?;
            let context = StandardContext::new();
            let result = tmpl.exec(&context)?;
            let cmp: Document = ($res).into();
            assert_ne!(result, cmp, "{} expression '{}' result -> {:?}", stringify!($name), $exp, result);
            Ok(())
        }
        test_expressions! {
            $( $tail )*
        }
    };
    () => {};
}

test_expressions! {
    // math
    basic_add: "1 + 2" == 3i64;
    basic_subtract: "4 - 2" == 2i64;
    basic_divide: "10 / 2" == 5i64;
    basic_multiply: "5 * 5" == 25i64;
    add_multiple: "3 + 3 + 3" == 9i64;
    sub_multiple: "9 - 3 - 3" == 3i64;
    divide_multiple: "125 / 5 / 5" == 5i64;
    multiply_multiple: "5 * 5 * 5" == 125i64;
    modulus: "12 % 5" == 2i64;
    order_left_to_right_1: "5+5+5+5+5/5" == 5i64;
    order_left_to_right_2: "20+5/5" != 21i64;
    order_with_inner_1: "20+ (5/5)" == 21i64;
    order_with_inner_2: "20+(5*5)" == 45i64;
    order_with_complex_inner: " 20+(5 + 5 + (2 + 1))" == 33i64;
    !fail_math_op_against_string: "'hello' + 5";

    // arrays
    get_index_of_array: "[1,2,3] | index(1)" == 2i64;
    expr_inside_array: "[1,(2+2),3] | index(1)" == 4i64;

    // maps
    get_mapping_key: "{'key' : 'value'} | key('key')" == "value";
    print_nested_mapping: "{ 'ley' : 'loo' , 'boom': { 'nested':1} } | string" == "{boom => {nested => 1},ley => loo}";

    // string interpolation
    upper_filter: "'Test' | upper" == "TEST";
    lower_filter: "'Test' | lower" == "test";
    case_sensitive_1: "'Test'" == "Test";
    case_sensitive_2: "'Test'" != "test";
    case_sensitive_3: "'Test'" != "TEST";
    math_op_to_string: "5 + 5 + 5 | string" == "15";
    trim_filter: "' hello ' | trim" == "hello";
    no_auto_trim_1: "' hello '" == " hello ";
    no_auto_trim_2: "' hello '" != "hello";
    concat_op: " 'hello ' ~ 'world'" == "hello world";
    concat_op_filter: " 'hello ' ~ 'world' | upper" == "HELLO WORLD";
    concat_multiple: "'one ' ~ 'two ' ~ 'three'" == "one two three";
    concat_non_string: "'one' ~ 1 ~ true" == "one1true";
    replace: "'this-is-a-thing'| replace('-','_') " == "this_is_a_thing";

    // encoding/decoding
    base64_encode_filter: "'Test' | base64" == "VGVzdA==";
    base64_decode_filter: "'VGVzdA==' | base64('decode')" == "Test";

    // scripts / commands
    script_key_filter_1: " script('echo -n test') | key('stdout') " == "test";
    script_key_filter_2: " script('echo -n test') | key('status') " == 0;
    command_var_args: "command('echo', '-n', 'test') | key('stdout')" == "test";

    // set
    test_set: "val = 'hello'" == "";
    test_set_return: "val = 'hello' ~ val" == "hello";

    // HTML Escape
    escape_html: "`<script>alert('hello!')</script>` | e" == "&lt;script&gt;alert(&#x27;hello!&#x27;)&lt;&#x2F;script&gt;";
}
