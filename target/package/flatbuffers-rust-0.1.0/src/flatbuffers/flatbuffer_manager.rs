#[macro_export]
macro_rules! register_struct_info {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }, $vec:expr) => {
        struct $name {}

        impl $name {
            fn get_fields_info<'a>(names_types: &mut Vec<StructInfo>) {
                let mut child_names_types = Vec::new();
                $( 
                    child_names_types.push(
                        FieldInfo {
                            field_name: stringify!($fname), 
                            field_type: stringify!($ftype)
                        }
                    ); 
                )*;
                let (prefix, struct_name) = stringify!($name).split_at(2);
                names_types.push(
                    StructInfo {
                        struct_name: struct_name,
                        fields: child_names_types
                    }
                );
            }
        }
        $name::get_fields_info(&mut $vec);
    };
}
// #[macro_export]
// macro_rules! register_instance_info {
//     () => ()
// }
// #[macro_export]
// macro_rules! find_field_position_vec {
//     ($name:ident :: $($field:ident).*, $fields:expr) => {
//         // let mut fields = Vec::new(); // vec<&str>
//         $fields.push(stringify!($name));
//         $( $fields.push(stringify!($field)); )*
//         // 先翻转 
//         $fields.reverse();
//         // $manager.field_position_vec(&mut fields);
//     }
// }
#[derive(Debug, Clone)]
pub struct StructInfo<'a> {
    pub struct_name: &'a str, // 结构体类型名
    pub fields: Vec<FieldInfo<'a>> // 结构体字段信息
}
impl<'a> StructInfo<'a> {
    pub fn find_field(&self, name: &str) -> Result<(&FieldInfo, usize), String> {
        let mut position = 0;
        let len = self.fields.len();
        let fields = self.fields.clone();
        for i in 0..len {
            if fields[i].field_name.eq(name) {
                return Ok((&self.fields[i], i+1))
            }
        }
        return Err(format!("NO field named {:?}", name));
    }
}
#[derive(Debug, Clone)]
pub struct FieldInfo<'a> {
    pub field_name: &'a str, // 字段名
    pub field_type: &'a str // 字段类型
}

#[derive(Debug)]
pub struct FlatBufferManager<'a> {
    // 存储所有结构体的信息
    pub struct_list: Vec<StructInfo<'a>>,
}
impl<'a> FlatBufferManager<'a> {
    pub fn new() -> FlatBufferManager<'a> {
        FlatBufferManager {
            struct_list: Vec::new()
        }
    }

    pub fn find_struct(&self, name: &str) -> Result<&StructInfo, String> {
        let mut struct_list = self.struct_list.clone();
        let len = struct_list.len();
        // println!("all structs is {}", len);
        for i in 0..len {
            if struct_list[i].struct_name.eq(name) {
                return Ok(&self.struct_list[i]);
            }
        }
        return Err(format!("NO struct named {:?}", name));
    }
    fn field_position_vec(&self, fields:&mut Vec<&str>) -> Result<Vec<usize>, String> {
        // 先获取到第一个 字段=类名
        let mut position_vec = Vec::new();
        let struct_name = fields.pop().unwrap();
        let struct_info = match self.find_struct(struct_name) {
            Ok(info) => info, // &structinfo类型 
            Err(e) => return Err(e),
        };
        // loop {
        // 使用 RefCell 变为& -> &mut 不可行
        // let mut rfc_struct_info = RefCell::new(struct_info);
        let field_name = fields.pop().unwrap();
        let rest_field = fields.len();
        // 去寻找对应的field
        let (field_type, p) = match struct_info.find_field(field_name) {
            Ok((field, p)) => (field.field_type, p),
            Err(e) => return Err(e),
        };
        position_vec.push(p);
        if rest_field == 0 {
            // 没有后面的内容了，说明成功，传数组
            position_vec.reverse();
            println!("翻转一次");
            return Ok(position_vec);
        }
        if field_type.eq("String") {
            return Err(format!("{:?} has not {:?}",struct_name, fields.pop().unwrap()));
        } else if field_type.eq("bool") {
            return Err(format!("{:?} has not {:?}",struct_name, fields.pop().unwrap()));
        } else {

            if field_type.starts_with("List<") {
                // 是 List<T>
                // 将 内部的 T 解析出来
                let (list_str, inner_type) = field_type.split_at(5);
                let mut inner_type_string = String::from(inner_type);// T>
                let last = inner_type_string.len() - 1;
                inner_type_string.remove(last);
                // 将 数字 加入vec
                // let index = fields.pop().unwrap();
                // position_vec.push(index.parse::<usize>());
                self.field_position_vec_inner(inner_type_string.as_str(), fields, &mut position_vec);
                return Ok(position_vec);
            
            } else {
                self.field_position_vec_inner(field_type, fields, &mut position_vec);
                return Ok(position_vec);
            }

            // 调用inner方法 不用翻转 在inner中翻转了
            // self.field_position_vec_inner(field_type, fields, &mut position_vec);
            // return Ok(position_vec);
        }
    }
    fn field_position_vec_inner(&self, struct_name: &str, fields:&mut Vec<&str>, position_vec: &mut Vec<usize>) -> Result<(), String> {
        
        let mut field_name = "";
        loop {
            field_name = fields.pop().unwrap();
            
            match field_name.parse::<usize>() {
                Ok(n) => { 
                    position_vec.push(n+1); 
                    // 考虑是否是最后一项
                    if fields.len() == 0 {
                        position_vec.reverse();
                        println!("翻转一次");
                        return Ok(());
                    } else {
                        continue; 
                    }
                }
                Err(e) => break,
            };
        }
        let struct_info = match self.find_struct(struct_name) {
            Ok(info) => info, // structinfo类型 
            Err(e) => return Err(e),
        };
        let rest_field = fields.len();
        // 去寻找对应的field
        let (field_type, p) = match struct_info.find_field(field_name) {
            Ok((field, p)) => (field.field_type, p),
            Err(e) => return Err(e),
        };
        position_vec.push(p);
        if rest_field == 0 {
            position_vec.reverse();
            println!("翻转一次");
            return Ok(());
        }
        if field_type.eq("String") {
            return Err(format!("{:?} has not {:?}",struct_name, fields.pop().unwrap()));
        } else if field_type.eq("bool") {
            return Err(format!("{:?} has not {:?}",struct_name, fields.pop().unwrap()));
        } else {
            // 在这里处理List<T> 和 T 的情况
            if field_type.starts_with("List<") {
                
                // 是 List<T>
                // 将 内部的 T 解析出来
                let (list_str, inner_type) = field_type.split_at(5);
                let mut inner_type_string = String::from(inner_type);// T>
                
                let last = inner_type_string.len() - 1;
                inner_type_string.remove(last);
                // println!("fields now is {:?}", fields);
                // 将 数字 加入vec
                // let index = fields.pop().unwrap();
                // position_vec.push(index.parse::<usize>());
                return self.field_position_vec_inner(inner_type_string.as_str(), fields, position_vec);
            } else {
                return self.field_position_vec_inner(field_type, fields, position_vec);
            }
        }
        // }
    }
    // 将&str 转为定位的&str数组
    pub fn to_local_vec(&self, local: &str, struct_name: &str) -> Result<Vec<String>, String> {
        let string = local.replace("[", ".");
        let string = string.as_str().replace("]", "."); // 可以优化
        let mut vec:Vec<&str> = string.as_str().split(".").collect();
        let mut string_vec = Vec::new();
        for e in vec {
            let e_s = String::from(e);
            if e_s.len() == 0 {
                continue;
            }
            string_vec.push(e_s);
        }
        string_vec.reverse();
        string_vec.push(String::from(struct_name));
        Ok(string_vec)
    }
    pub fn field_position(&self, fields:&mut Vec<String>) -> Result<Vec<usize>, String> {
        // 先将 Vec<String> 转为 Vec<&str>
        let mut vec:Vec<&str> = Vec::new();
        for i in 0..fields.len() {
            vec.push(fields[i].as_str());
        }
        self.field_position_vec(&mut vec)
    }
}