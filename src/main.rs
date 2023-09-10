use exmex;
use std::collections::HashMap;

fn main() {
    println!("{}", equation_gen(equation_mask_gen(vec!["+", "-", "*", "/", "^"], true, 10).as_str(), -100, 100, false));
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Int(i32),
    Float(f32),
}

impl Type {
    fn int(self) -> i32 {
        if let Type::Int(c) = self {
            c
        } else {
            panic!("Not a Int");
        }
    }

    fn float(self) -> f32 {
        if let Type::Float(c) = self {
            c
        } else {
            panic!("Not a Float");
        }
    }
}

//todo: добавить аргумент словарь в котором будет указана сложность для операторов
fn equation_gen(mask: &str, start: i32, end: i32, float: bool) -> String {
    let mut result: String = "".to_string();
    let mut equation: String = mask.clone().to_owned();
    let mut num: Type;
    let mut x: String = "".to_string();

    let mut alphabet2 = (b'A'..=b'z')
        .map(|c| {
            if c.is_ascii_lowercase() {
                (c as char, Type::Int(fastrand::i32(start..end)))
            } else {
                (c as char, Type::Int(fastrand::i32(0..end)))
            }
        })
        .filter(|c| c.0.is_alphabetic())
        .collect::<HashMap<_, _>>();
    let mut alphabet_up = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"
        .chars()
        .map(|c| (c as char, Type::Int(fastrand::i32(0..end))))
        .collect::<HashMap<_, _>>();

    let mut alphabet = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя"
        .chars()
        .map(|c| (c as char, Type::Int(fastrand::i32(start..end))))
        .collect::<HashMap<_, _>>();

    if float {
        alphabet = (b'A'..=b'z')
            .map(|c| {
                if c.is_ascii_lowercase() {
                    (
                        c as char,
                        Type::Float(fastrand::f32() * fastrand::i32(start..end) as f32),
                    )
                } else {
                    (
                        c as char,
                        Type::Float(fastrand::f32() * fastrand::i32(0..end) as f32),
                    )
                }
            })
            .filter(|c| c.0.is_alphabetic())
            .collect::<HashMap<_, _>>();

        alphabet_up = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"
            .chars()
            .map(|c| (
                c as char,
                Type::Float(fastrand::f32() * fastrand::i32(0..end) as f32),
            ))
            .collect::<HashMap<_, _>>();

        alphabet = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя"
            .chars()
            .map(|c|(
                c as char,
                Type::Float(fastrand::f32() * fastrand::i32(start..end) as f32),
            ))
            .collect::<HashMap<_, _>>();
    }
    alphabet.extend(alphabet_up);
    alphabet.extend(alphabet2);

    let alphabet_vec2: Vec<char> = (b'A'..=b'z')
    .map(|c| c as char)
    .filter(|c| c.is_alphabetic())
    .collect::<Vec<_>>();

    let mut alphabet_vec: Vec<char> = "абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"
        .chars()
        .collect::<Vec<_>>();

    alphabet_vec.extend(alphabet_vec2);

    let mut difficult: usize = 0;

    for sym in alphabet_vec {
        num = *alphabet.get(&sym).unwrap();
        if sym != 'x' {
            if float {
                x = num.float().to_string();
                if mask.contains(sym) {
                    difficult += x.len();
                }
                equation = equation.replace(sym, &x);
                difficult += x.len();
            } else {
                x = num.int().to_string();
                if mask.contains(sym) {
                    difficult += x.len();
                }
                equation = equation.replace(sym, &x);
            }
        } else {
            result = equation.clone();
            if float {
                equation = equation.replace(sym, &num.float().to_string());
                x = num.float().to_string();
            } else {
                equation = equation.replace(sym, &num.int().to_string());
                x = num.int().to_string();
            }
            println!("{}\n\n\n\n\n", x);
        }
    }
    result.push_str(" = ");
    result.push_str(&exmex::eval_str::<f64>(&equation.replace(";", "abs").replace("@", ")")).unwrap().to_string());
    println!("difficult: {}", difficult);
    result.replace(";(", "|").replace("@", "|")
}

//todo: сделать преобразование *-... и тд в *(-...)
fn equation_mask_gen(ops: Vec<&str>, parens: bool, len: usize) -> String {
    let mut last_elem: usize;
    let mut mask: String = "#".to_string();
    let mut idx_count: usize = 0;
    let mut paren_list: Vec<usize> = Vec::new();
    let mut used_syms: Vec<char> = Vec::new();
    let ops_len: usize = ops.len() - 1;
    let mut push_paren: bool = false;
    let mut len_in_parens: usize = 0;
    let mut last_sym: char;
    let mut if_op: bool;

    let alphabet_vec2: Vec<char> = (b'A'..=b'z')
    .map(|c| c as char)
    .filter(|c| c.is_alphabetic())
    .collect::<Vec<_>>();

    let mut alphabet_vec: Vec<char> = "абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"
        .chars()
        .collect::<Vec<_>>();

    alphabet_vec.extend(alphabet_vec2);

    while mask.chars().count() <= len {
        last_sym = mask.chars().collect::<Vec<char>>()[mask.chars().count() - 1];
        if last_sym == '^' {
            mask.push_str(fastrand::u8(2..3).to_string().as_str());
            continue;
        }

        if fastrand::u8(0..100) < 65 {
            if last_sym == '-' || last_sym == '+' {
                mask.push_str(&alphabet_vec[idx_count].to_string().as_str().to_uppercase());
                used_syms.push(alphabet_vec[idx_count].to_string().as_str().to_uppercase().chars().next().unwrap());
                len_in_parens += 1;
            } else {
                mask.push(alphabet_vec[idx_count]);
                used_syms.push(alphabet_vec[idx_count]);
                len_in_parens += 1;
            }
            idx_count += 1;
            if push_paren {
                if *paren_list.last().unwrap() == 0 {
                    mask.push(')');
                    paren_list.pop();
                } else {
                    mask.push('@'); // @ - ) (for abs)
                    paren_list.pop();
                }
                len_in_parens = 1;
                push_paren = false;
                mask.push_str(ops[fastrand::usize(0..ops_len)]);
            } else {
                mask.push_str(ops[fastrand::usize(0..ops_len)]);
                len_in_parens += 1;
            }
        } else if !(last_sym == '(') && mask.len() > 1 {
            if len_in_parens > 2 {
                if fastrand::u8(0..100) > 50 {
                    mask.push('(');
                    paren_list.push(0);
                } else {
                    mask.push_str(";$");    // ; - abs, $ - (    (  abs(  )
                    paren_list.push(1);
                }
                len_in_parens = 0;
            }
        }

        if fastrand::u8(0..100) > 50 {
            if paren_list.len() > 0 {
                if len_in_parens > 2 {
                    push_paren = true;
                }
            }
        }
 
    }

    for _ in 0..paren_list.len() {
        last_sym = mask.chars().collect::<Vec<char>>()[mask.chars().count() - 1];
        if_op = !ops.contains(&last_sym.to_string().as_str());
        if if_op && !(last_sym == '(' || last_sym == '$') {
            if paren_list.last().unwrap() == &0 {
                mask.push(')')
            } else {
                mask.push('@')
            }
            paren_list.pop();
        } else {
            mask.push(alphabet_vec[idx_count]);
            idx_count += 1;
            mask.push_str(ops[fastrand::usize(0..ops_len)]);
            if mask.chars().collect::<Vec<char>>()[mask.chars().count() - 1]  == '-' {
                mask.push_str(&alphabet_vec[idx_count].to_string().as_str().to_uppercase());
            } else {
                mask.push(alphabet_vec[idx_count]);
            }
            if paren_list.last().unwrap() == &0 {
                mask.push(')')
            } else {
                mask.push('@')
            }
            idx_count += 1;
            paren_list.pop();
        }
    }
    last_sym = mask.chars().collect::<Vec<char>>()[mask.chars().count() - 1];
    if ops.contains(&last_sym.to_string().as_str()) {
        mask.push_str(&alphabet_vec[idx_count].to_string().as_str().to_uppercase());
    };
    println!("{}", mask.replace(used_syms[fastrand::usize(0..used_syms.len() - 1)], "x").replace("$", "(").replace("#", ""));
    mask.replace(used_syms[fastrand::usize(0..used_syms.len() - 1)], "x").replace("$", "(").replace("#", "")
}