#[macro_use]
extern crate add_getters_setters;

#[derive(AddGetter, AddGetterMut, AddSetter)]
struct Ts {
    jaf: u8,
    
    #[set]
    field_1: u8,

    #[get]
    #[get_mut]
    field_2: String,
}

// these functions shouldn't be set since there are not attrs on jaf. if they are set then it wont compile because these would be duplicate function definitions, so then we'd know theres something wrong.
impl Ts {
    #[allow(dead_code)]
    pub fn get_jaf(&self) -> & u8 {
        &self.field_1
    }

    #[allow(dead_code)]
    pub fn get_jaf_mut(&mut self) -> &mut u8 {
        &mut self.field_1
    }

    #[allow(dead_code)]
    pub fn set_jaf(&mut self, v: u8) {
        self.jaf = v;
    }
}

#[test]
fn test_add_setter() {
    let mut a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
    a.set_field_1(14);
    assert_eq!(a.field_1, 14);
}

#[test]
#[should_panic]
fn test_add_setter_should_panic() {
    let mut a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
    a.set_field_1(20);
    assert_eq!(a.field_1, 11);
}

#[test]
fn test_add_getter() {
    let a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
    assert_eq!(a.get_field_2(), &String::from("hello"));
}

#[test]
fn test_add_getter_mut() {
    let mut a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
    let b = a.get_field_2_mut();
    *b = String::from("world");
    assert_eq!(a.get_field_2(), &String::from("world"));
}

#[test]
#[should_panic]
fn test_add_getter_mut_should_panic() {
    let mut a = Ts {jaf: 4, field_1: 0, field_2: String::from("hello")};
    let b = a.get_field_2_mut();
    *b = String::from("world");
    assert_eq!(a.get_field_2(), &String::from("hello"));
}