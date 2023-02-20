mod chapter_1;
mod chapter_2;

fn main() {
    crate::chapter_1::hello_world::greet_world();
    chapter_1::hello_world::penguin();
    self::chapter_2::basic_type::basic_concept();
    chapter_2::basic_type::assert_try();
    chapter_2::basic_type::int_overflow();
    chapter_2::basic_type::range_demo();
    chapter_2::basic_type::complex_demo();
    chapter_2::basic_type::char_demo();
    chapter_2::ownership_reference::ownership_demo();
    chapter_2::ownership_reference::mut_demo();
    chapter_2::ownership_reference::mut_reference_demo();
    chapter_2::compound_type::string_slice_demo();
    chapter_2::compound_type::array_slice_demo();
    chapter_2::compound_type::string_str_demo();
    chapter_2::compound_type::operate_string();
    chapter_2::compound_type::tuple_demo();
    chapter_2::compound_type::struct_demo();
    chapter_2::compound_type::enum_demo();
    chapter_2::compound_type::array_demo();
    chapter_2::process_control::if_let_demo();
    chapter_2::process_control::matches_demo();
    chapter_2::generic_trait::trait_demo();
    chapter_2::generic_trait::trait_demo2();
    chapter_2::generic_trait::trait_demo3();
    chapter_2::generic_trait::trait_demo4();
    chapter_2::generic_trait::trait_demo5();
    chapter_2::generic_trait::trait_object_demo();
    chapter_2::generic_trait::trait_with_default_generic_type_args_demo();
    chapter_2::generic_trait::same_name_method_demo();
    chapter_2::generic_trait::trait_constraint_of_trait_demo();
    chapter_2::vec_hashmap::vec_demo();
    chapter_2::vec_hashmap::hashmap_demo();
    chapter_2::type_conversion::type_cast_demo();
    chapter_2::exception::exception_demo();
}