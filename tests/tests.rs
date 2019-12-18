#[macro_use]
extern crate add_getters_setters;

#[derive(AddSetter, AddGetter, AddGetterMut)]
struct Ts {
    #[set]
    field_1: u8,

    #[get]
    #[get_mut]
    field_2: String,
}

#[test]
fn test_add_setter() {
    let mut a = Ts {field_1: 0, field_2: String::from("hello")};
    a.set_field_1(14);
    assert_eq!(a.field_1, 14);
}

#[test]
#[should_panic]
fn test_add_setter_should_panic() {
    let mut a = Ts {field_1: 0, field_2: String::from("hello")};
    a.set_field_1(20);
    assert_eq!(a.field_1, 11);
}

#[test]
fn test_add_getter() {
    let a = Ts {field_1: 0, field_2: String::from("hello")};
    assert_eq!(a.get_field_2(), &String::from("hello"));
}

#[test]
fn test_add_getter_mut() {
    let mut a = Ts {field_1: 0, field_2: String::from("hello")};
    let b = a.get_field_2_mut();
    *b = String::from("world");
    assert_eq!(a.get_field_2(), &String::from("world"));
}

#[test]
#[should_panic]
fn test_add_getter_mut_should_panic() {
    let mut a = Ts {field_1: 0, field_2: String::from("hello")};
    let b = a.get_field_2_mut();
    *b = String::from("world");
    assert_eq!(a.get_field_2(), &String::from("hello"));
}