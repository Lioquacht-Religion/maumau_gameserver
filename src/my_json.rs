use std::{collections::HashMap, error::Error, fmt::{Display, format}};

pub enum Keyvalue{
    Value(String),
    List(Vec<Keyvalue>),
    Object(JSONObject),
}

impl Keyvalue{
    pub fn to_string(&self) -> String{
        match self{
            Self::Value(s) => format!("\"{}\"", &s),
            Self::List(l) => {
                /*let mut list_string = String::new();
                l.iter().for_each(|val| {
                    list_string.push_str(&format!("{}, ", &val.to_string()) )
                });*/
                format!("[{}]", JSONObject::comma_sep(l))
            },
            Self::Object(obj) => {
                obj.to_string()
            },
        }
    }

    pub fn get_value(&self) -> Option<&str>{
        if let Keyvalue::Value(val) = self{
            Some(val)
        }
        else{ None }
    }
}

impl Display for Keyvalue{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct JSONObject{
    key_values : HashMap<String, Keyvalue>
}

impl JSONObject{

    pub fn new() -> Self{

        Self{
            key_values : HashMap::new(),
        }
    }

    pub fn add(&mut self, key : String, value : Keyvalue){
        self.key_values.insert(key, value);
    }

    pub fn get_ref(&self, key : &str) -> Option<&Keyvalue>{
        self.key_values.get(key)
    }

    pub fn get_mut(&mut self, key : &str,) -> Option<&mut Keyvalue>{
        self.key_values.get_mut(key)
    }

    pub fn to_string(&self) -> String{
                let mut obj_string = String::new();
                let mut val_iter = self.key_values.iter();

                if let Some(key_val) = val_iter.next(){
                    let mut key_val = key_val;
                loop{
                    obj_string.push_str(
                        &format!(
                            "\"{}\" : {}",
                            &key_val.0, &key_val.1.to_string()
                            )
                        );
                    if let Some(key_val_next) = val_iter.next() {
                        obj_string.push_str(",\n ");
                        key_val = key_val_next;
                    }
                    else{
                        obj_string.push('\n');
                        break;
                    }
                }
                }


                format!("{{ {} }}", obj_string)
    }

    pub fn comma_sep(list : &Vec<Keyvalue>)
        -> String{
                let mut obj_string = String::new();
                let mut val_iter = list.iter();

                if let Some(key_val) = val_iter.next(){
                    let mut key_val = key_val;
                loop{
                    obj_string.push_str(&key_val.to_string());
                    if let Some(key_val_next) = val_iter.next() {
                        obj_string.push_str(",");
                        key_val = key_val_next;
                    }
                    else{
                        break;
                    }
                }
                }
                obj_string
    }

    pub fn from_string(value : &str) -> Result<JSONObject, ()>{

        match Self::parse_obj(value.replace("\n", "").as_str()) {
            Some((obj, text)) => Ok(obj),
            None => Err(()),
        }
    }

    pub fn parse_obj(value : &str) -> Option<(JSONObject, &str)>{
        let mut json = JSONObject::new();

        if value.len() <= 0 {
            return Some((json, value));
        }

        if let Some(innerobj)= Self::inside_swirly_braces(value){
            let mut value = innerobj;
        loop{
            let (key, _st, end) = Self::inside_apos(value);
            value = &value[end..value.len()];
            if let Some(mut after_key) = Self::after_colon(value){
                if let Some((key_val, rest)) = Self::parse_value(after_key.trim_start()){
                    json.add(key.into(), key_val);
                    after_key = rest;
                }
                if let Some(after_comma) = Self::after_comma(after_key){
                    value = after_comma;
                }
                else{
                    return Some((json, value));
                }
            }
            else{
                return Some((json, value));
            }
        }
        }
        None

        //json

    }

    pub fn parse_value(start_value : &str) -> Option<(Keyvalue, &str)> {
        if let Some(c) = start_value.chars().next(){
            match c {
                '{' => {
                    if let Some((obj, innerobj)) = Self::parse_obj(start_value){
                    return Some(
                        (
                            Keyvalue::Object(obj),
                            &start_value[innerobj.len()+1..start_value.len()]
                        )
                    );
                    }
                                    },
                '"' => {
                    let (innerval, _l_start, l_end) = Self::inside_apos(start_value);
                    let value = &start_value[l_end..start_value.len()];

                    //println!("key: , val: {}", value);
                    return Some((Keyvalue::Value(innerval.into()), value));
                },
                '[' => {
                    if let Some((list, innervalues)) = Self::parse_list(start_value){
                        return Some((
                        Keyvalue::List(list),
                        &start_value[innervalues.len()+1..start_value.len()]
                        ));
                    }

                },
                _ => {},
            }
        }
        None
    }

    pub fn parse_list(value: &str) -> Option<(Vec<Keyvalue>, &str)>{
        let mut list : Vec<Keyvalue> = Vec::new();

        if let Some(innervalues) = Self::inside_brackets(value){
            let mut out_rest = innervalues;
        loop{
                if let Some((key_val, rest)) = Self::parse_value(out_rest.trim_start()){
                    list.push(key_val);
                    out_rest = rest;
                }// needs to return innerstring, or bounderies
                if let Some(after_comma) = Self::after_comma(out_rest){
                    out_rest = after_comma;
                }
                else{
                    return Some((list, innervalues));
                }

        }
        }
        None
    }

    pub fn after_colon(value : &str) -> Option<&str>{
        Self::after_char(':', value)
    }
    pub fn after_comma(value : &str) -> Option<&str>{
        Self::after_char(',', value)
    }

    pub fn after_char(after_char : char, value : &str) -> Option<&str>{
        let mut chars = value.chars();
        let mut counter : usize = 0;
        while let Some(c) = chars.next(){
                if c == after_char{
                    return
                        Some(&value[counter+1..value.len()]);
                }
                else{}
            counter += 1;
        }
        None
    }

    pub fn inside_swirly_braces(value : &str) -> Option<&str>{
        Self::inside_enclosing_chars('{', '}', value)
    }
    pub fn inside_brackets(value : &str) -> Option<&str>{
        Self::inside_enclosing_chars('[', ']', value)
    }

    pub fn inside_enclosing_chars(c1 : char, c2 : char, value : &str) -> Option<&str>{
        //TODO: error return handling
        let mut chars = value.chars();
        //let mut end : usize = 0;
        let mut counter : usize = 0;
        let mut f_inner_char : usize = 0;
        let mut b_inner_char : usize = 0;
        while let Some(c) = chars.next(){
                if c == c1 {
                    let start : usize= counter;
                    counter += 1;
                    while let Some(c) = chars.next(){
                            if c == c1 { f_inner_char += 1 }
                            else
                            if c == c2 {
                                //end = counter;
                                if f_inner_char == b_inner_char {
                                    return Some(&value[start+1..counter]);
                                }
                                else{
                                    b_inner_char += 1;
                                }
                            }
                            else {}

                        counter += 1;
                    }
                    //return &value[start+1..end];
                    return None;
                }
                else{}

            counter += 1;
        };

        None
    }

    #[allow(unused_assignments)]
    pub fn inside_apos(value : &str) -> (&str, usize, usize) {
        let mut chars = value.chars();
        let mut start : usize = 0;
        //let mut end : usize = 0;
        let mut counter : usize = 0;

        while let Some(c) = chars.next(){
            match c {
                '"' => {
                    start = counter;
                    counter += 1;
                    while let Some(c) = chars.next(){
                        match c {
                            '"' => {
                                return (&value[start+1..counter], start, counter+1);
                            },
                            _ => {},
                        }
                        counter += 1;

                    }
                },
                _ => {},
            }
            counter += 1;

        }

        (value, 0, 0)

    }

}

impl TryFrom<&str> for JSONObject{
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

//#[cfg(test)]
pub mod tests{
    use crate::my_json::JSONObject;
    use crate::my_json::Keyvalue::{*, self};


  //  #[test]
    pub fn test_json_to_string(){
        let test_string =
            "asddf  {\n
            \"attr1\" : \"123\",\n
            \"attr10\" : \"123\",\n
            \"attr11\" : \"123\",\n

            \"attr3\" : {\n
            \"i_attr4\" : \"456\",
            \"i_attr5\" : \"456\"
            \n},\n

            \"attr6\" : \"123\",\n
            \"attr7\" : \"123\",\n


            \"attr23\" : \"123\",\n
            \"attr2\" : [\"123\", \"123\",],\n
            \"attr24\" : \"123\"\n
            }  ehtdhj";


        println!("{test_string}");

        let mut test_json = JSONObject::new();
        test_json.add("attr1".to_string(), Value("123".to_string()));

        test_json.add("attr2".to_string(),
                      List(vec![
                                                Value("123".to_string()),
                                                Value("123".to_string()),
        ]));

        for i in 0..1{
             test_json.add(format!("attrloop{}", i), Value("123".to_string()));
             let mut l_jsobj = JSONObject::new();
             let mut l_jslist : Vec<Keyvalue> = vec![];
             for j in 0..1{
                  l_jsobj.add(format!("attrloop{}", j), Value("123".to_string()));
                  l_jslist.push(Value("123".to_string()));
             }
             test_json.add(format!("objloop{}", i), Object(l_jsobj));
             test_json.add(format!("listloop{}", i), List(l_jslist));
        }

        test_json.add("attr3".to_string(), Object(JSONObject::new()));

        println!("\n json to string: {}", test_json.to_string());
        //assert!(test_json.to_string(), test_string);
        //
        //test string to JSON

        /*println!("{}", JSONObject::inside_swirly_braces(
                JSONObject::inside_swirly_braces(test_string)
                ));
                */
        let t = JSONObject::from_string(test_json.to_string().as_str()).unwrap().to_string();

        println!("\n string to json: {}", t );

    }
}
