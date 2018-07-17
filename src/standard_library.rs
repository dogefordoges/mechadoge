use std::collections::HashMap;
use processor::Snack;

pub fn bark(value: String, global_variables: &HashMap<String, Snack>, string_heap: &HashMap<String, Snack>) {

    let mut print_value: String = value.clone();

    if print_value.contains("GLOBAL") {
        print_value = global_variables.get(&print_value).unwrap().to_string();
    }

    if print_value.contains("STR") {
        println!("{}", print_value);
        print_value = string_heap.get(&print_value).unwrap().to_string();
    }

    println!("{}", print_value);
}

pub fn add(v1: &Snack, v2: &Snack) -> Snack {
    match v1 {
        Snack::INT(i1) => {
            match v2 {
                Snack::INT(i2) => {
                    Snack::INT(i1 + i2)
                },
                _ => {
                    panic!("Numeric values given to add must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::UINT(u1) => {
            match v2 {
                Snack::UINT(u2) => {
                    Snack::UINT(u1 + u2)
                },
                _ => {
                    panic!("Numeric values given to add must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::FLOAT(f1) => {
            match v2 {
                Snack::FLOAT(f2) => {
                    Snack::FLOAT(f1 + f2)
                },
                _ => {
                    panic!("Numeric values given to add must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        _ => {
            panic!("Only numeric values allowed as input to add. Found: {:?} {:?}", v1, v2);
        }
    }
}

pub fn sub(v1: &Snack, v2: &Snack) -> Snack {
    match v1 {
        Snack::INT(i1) => {
            match v2 {
                Snack::INT(i2) => {
                    Snack::INT(i1 - i2)
                },
                _ => {
                    panic!("Numeric values given to sub must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::UINT(u1) => {
            match v2 {
                Snack::UINT(u2) => {
                    Snack::UINT(u1 - u2)
                },
                _ => {
                    panic!("Numeric values given to sub must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::FLOAT(f1) => {
            match v2 {
                Snack::FLOAT(f2) => {
                    Snack::FLOAT(f1 - f2)
                },
                _ => {
                    panic!("Numeric values given to sub must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        _ => {
            panic!("Only numeric values allowed as input to sub, found: {:?} {:?}", v1, v2);
        }
    }
}

pub fn mul(v1: &Snack, v2: &Snack) -> Snack {
    match v1 {
        Snack::INT(i1) => {
            match v2 {
                Snack::INT(i2) => {
                    Snack::INT(i1 * i2)
                },
                _ => {
                    panic!("Numeric values given to mul must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::UINT(u1) => {
            match v2 {
                Snack::UINT(u2) => {
                    Snack::UINT(u1 * u2)
                },
                _ => {
                    panic!("Numeric values given to mul must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::FLOAT(f1) => {
            match v2 {
                Snack::FLOAT(f2) => {
                    Snack::FLOAT(f1 * f2)
                },
                _ => {
                    panic!("Numeric values given to mul must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        _ => {
            panic!("Only numeric values allowed as input to mul, found: {:?} {:?}", v1, v2);
        }
    }
}

pub fn div(v1: &Snack, v2: &Snack) -> Snack {
    match v1 {
        Snack::INT(i1) => {
            match v2 {
                Snack::INT(i2) => {
                    Snack::INT(i1 / i2)
                },
                _ => {
                    panic!("Numeric values given to div must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::UINT(u1) => {
            match v2 {
                Snack::UINT(u2) => {
                    Snack::UINT(u1 / u2)
                },
                _ => {
                    panic!("Numeric values given to div must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        Snack::FLOAT(f1) => {
            match v2 {
                Snack::FLOAT(f2) => {
                    Snack::FLOAT(f1 / f2)
                },
                _ => {
                    panic!("Numeric values given to div must be of same type, found: {:?} {:?}", v1, v2);
                }
            }
        },
        _ => {
            panic!("Only numeric values allowed as input to div, found: {:?} {:?}", v1, v2);
        }
    }
}

pub fn equal(v1: &Snack, v2: &Snack) -> Snack {
    match v1 {
        Snack::UINT(u1) => {
            match v2 {
                Snack::UINT(u2) => {
                    Snack::BOOLEAN(u1 == u2)
                },
                _ => { panic!("Values given to `is` must be of same type, found: {:?} {:?}", v1, v2); }
            }
        },
        Snack::INT(i1) => {
            match v2 {
                Snack::INT(i2) => {
                    Snack::BOOLEAN(i1 == i2)
                },
                _ => { panic!("Values given to `is` must be of same type, found: {:?} {:?}", v1, v2); }
            }
        },
        Snack::FLOAT(f1) => {
            match v2 {
                Snack::FLOAT(f2) => {
                    Snack::BOOLEAN(f1 == f2)
                },
                _ => { panic!("Values given to `is` must be of same type, found: {:?} {:?}", v1, v2); }
            }
        },
        Snack::STRING(s1) => {
            match v2 {
                Snack::STRING(s2) => {
                    Snack::BOOLEAN(s1 == s2)
                },
                _ => { panic!("Values given to `is` must be of same type, found: {:?} {:?}", v1, v2); }
            }
        },
        Snack::BOOLEAN(b1) => {
            match v2 {
                Snack::BOOLEAN(b2) => {
                    Snack::BOOLEAN(b1 == b2)
                },
                _ => { panic!("Values given to `is` must be of same type, found: {:?} {:?}", v1, v2); }
            }
        }        
    }
}
