// bool -> FlatBufferVar
// String -> FlatBufferVar
// list -> FlatBufferVar
// 结构体 -> FlatBufferVar
// 不用考虑 SEPARATOR

// message = Vec<FlatBufferVar> + name
// messages = Vec<message>
use std::any::Any;

use flatbuffers::flatbuffer::FlatBuffer;
use flatbuffers::flatbuffer_var::{ FlatBufferVar, FlatBufferType };
use flatbuffers::flatbuffer_handler::FlatBufferHandler;

static SEPARATOR: u8 = 0xff;

#[derive(Debug)]
pub struct FlatBufferHelper {

}
// 全是基本类型的转化
// 对 整个data做转化，不考虑类型是否匹配
impl FlatBufferHelper {
    pub fn from_bool(&self, data: bool) -> Option<FlatBufferVar> {
        let mut data_vec = vec![0u8;1];
        if data {
            data_vec = vec![1u8;1];
        }
        FlatBufferVar::new( FlatBufferType::FBBool, 
                            Some(data_vec), None, None, None )
    }
    pub fn from_string(&self, data: String) -> Option<FlatBufferVar> {
        let mut data_vec = data.into_bytes();
        FlatBufferVar::new( FlatBufferType::FBString, 
                            Some(data_vec), None, None, None )
    }
    // rust 原始数据，不做任何改动 转为对应的FlatBufferVar
    pub fn from_vec<T>( &self, data:&mut Vec<T>, e_type: u8) 
                        -> Option<FlatBufferVar> where T: Any
    {
        // 通过反射机制，得到T的类型  或者 增加参数，告知传入的类型
        let mut flatbuffer_list = Vec::new();
        if e_type == 0u8 {
            // bool
            for len in 0..data.len() {
                // let element = data[len];
                let value_any = &data[len] as &Any;
                let value_bool = match value_any.downcast_ref::<bool>() {
                    Some(as_bool) => *as_bool,
                    None => continue, // 可能出错了！
                };
                // println!("添加");
                flatbuffer_list.push(self.from_bool(value_bool).unwrap());
            }
        } else if e_type == 1u8 {
            // string
            for len in 0..data.len() {
                // let element = data[len];
                let value_any = &data[len] as &Any;
                let value_string = match value_any.downcast_ref::<String>() {
                    Some(as_string) => as_string.as_str().to_string(),
                    None => continue, // 可能出错了
                };
                flatbuffer_list.push(self.from_string(value_string).unwrap());
            }
        } else if e_type == 2u8 {
            // list struct 合并
            // 属于FlatBufferVar类型 不做操作，直接push list中不能有None
            for len in 0..data.len() {
                // let element = data[len];
                let value_any = &data[len] as &Any;
                let value_flatbuffer_var = match value_any.downcast_ref::<FlatBufferVar>() {
                    Some(ref as_flatbuffervar) => (*as_flatbuffervar).clone(), // 为什么这样可以？
                    None => continue, // 可能出错了
                };
                // println!("value_flatbuffer_var: {:?}", value_flatbuffer_var);
                flatbuffer_list.push(value_flatbuffer_var);
            }
        }
        FlatBufferVar::new( FlatBufferType::FBList, None, 
                            Some(data.len() as u8),
                            Some(flatbuffer_list), None )
    }

    // 可能不是特别好用 数据可以再进行操作
    pub fn from_flatbuffer(&self, data:&mut FlatBuffer) -> Option<FlatBufferVar> {
        let (bytes, pivot) = data.bytes();
        // println!("本次bytes is {:?}", bytes);
        FlatBufferVar::new( FlatBufferType::FBStruct, 
                            bytes, None, None, Some(pivot as u32) )
    }
    pub fn to_string(&self, data: &Vec<u8>, root: usize) -> String {
        let end = data.len();
        String::from_utf8(data[root..end].to_vec()).unwrap()
    }
    pub fn to_bool(&self, data: &Vec<u8>, root: usize) -> bool {
        // 错误处理可以更好
        if data[root] == 0u8 {
            return false;
        } else {
            return true;
        }
    }
    // 专门为数组准备的 数组中不存在None
    fn to_vec(&self, data: &Vec<u8>, root: usize) -> Result<Vec<Vec<u8>>, String> {
        let handler = FlatBufferHandler{};
        let mut bytes_list = Vec::new();

        // 先获取到数组长度
        let vec_len = data[root] as u32;
        for p in 0..vec_len {
            // 获取每一个字段的bytes
            let (bytes, pivot) = match handler.get_part_data(1 + p as usize, root, data) {
                Ok((bytes, pivot)) => (bytes, pivot),
                Err(e) => return Err(e),
            };
            bytes_list.push(bytes.unwrap());
        }
        Ok(bytes_list)
    }
    pub fn to_vec_string(&self, data: &Vec<u8>, root: usize) -> Result<Vec<String>, String> {
        let bytes_list = match self.to_vec(data, root) {
            Ok(list) => list,
            Err(e) => return Err(e),
        };
        let mut string_list = Vec::new();
        for bytes in bytes_list {
            string_list.push(self.to_string(&bytes, 1));
        }
        Ok(string_list)
    }
    // 所有字段分离出来，存入Vec 字段可能为空
    pub fn to_struct(&self, data: &Vec<u8>, root: usize) -> Result<Vec<Option<Vec<u8>>>, String> {
        let handler = FlatBufferHandler{};
        let mut fields = Vec::new();
        // 先判断是否是原始类型
        if data[root -1] == SEPARATOR {
            // 属于原始类型
            return Err(format!("{:?} is a primitive type", data));
        }
        // 开始分离

        // 先获取到数组长度
        let vec_len = data[root] as u32;
        for p in 0..vec_len {
            // 获取每一个字段的bytes
            let (bytes, pivot) = match handler.get_part_data(1 + p as usize, root, data) {
                Ok((bytes, pivot)) => (bytes, pivot),
                Err(e) => return Err(e),
            };
            fields.push(bytes);
        }
        Ok(fields)
    }
    // pub fn to_struct_with_manager(&self, data: &Vec<u8>, root: usize, manager: &mut FlatBufferManager)
}