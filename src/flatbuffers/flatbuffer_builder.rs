// 将数据 转化为flatbuffer data部分，并且更新vtable
// 转化过程先定串行
// 以后可以扩展为异步转化, 所有数据同时转化后，进行拼接+更新vtable 
use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

use flatbuffers::flatbuffer::FlatBuffer;
use flatbuffers::flatbuffer_var::{ FlatBufferVar, FlatBufferType };
#[macro_export]
macro_rules! push_null {
    ({ $( $field:expr, )* }, $helper:expr, $fields:ident ) => {
        $(
            $fields.push($field);
        )*;
    };
}
#[macro_export]
macro_rules! push_vec {
    ({ $( $field:expr, $ftype:expr,)* }, $helper:expr, $fields:ident) => {
        $(
            $fields.push($helper.from_vec(&mut $field, $ftype));
        )*;
    };
}
#[macro_export]
macro_rules! push_bool {
    ({ $( $field:expr, )* }, $helper:expr, $fields:ident) => {
        $(
            $fields.push($helper.from_bool($field));
        )*;
    };
}
#[macro_export]
macro_rules! push_string {
    ({ $( $field:expr, )* }, $helper:expr, $fields:ident) => {
        $(
            $fields.push($helper.from_string($field));
        )*;
    };
}
#[macro_export]
macro_rules! push_flatbuffervar {
    ({ $( $field:expr, )* }, $helper:expr, $fields:ident) => {
        $(
            $fields.push($helper.from_flatbuffer(&mut $field));
        )*;
    };
}
#[macro_export]
// 返回 Result<>
macro_rules! build_flatbuffer_or_error {
    ({ $( $var:expr,)*} $builder:ident ) => {
        {
            let mut fields = Vec::new();
            $( fields.push($var); )*;
            let len = Some(fields.len() as u8);
            // 生成flatbuffer
            $builder.build(&mut fields, len)
        }
        
    };
}
static SEPARATOR: u8 = 0xff;
#[derive(Debug)]
struct MemberInfo {
    // String bool List<T> T 
    member_type: String,
    // 至少有一项内容
    member_value: Vec<Vec<u8>>
}
#[derive(Debug)]
pub struct InstanceInfo {
    instance_name: String, 
    instance_type: String, // 类型一定都是结构体 或 List
    member_info: Vec<Option<MemberInfo>> // MemberInfo 没有pub can't leak private type
}
#[derive(Debug)]
pub struct FlatBufferBuilder {
    // 可是设置 slot的大小 防止浪费过大的空间
    // 最大长度？
    // 存储所有参与转化的实例信息 只有结构体才能被注册
    instance_list: Vec<InstanceInfo>
}

impl FlatBufferBuilder {
    pub fn new() -> FlatBufferBuilder {
        FlatBufferBuilder {
            instance_list: Vec::new()
        }
    }
    
    // 所有调用的方法都要更新flatbuffer的vtable部分

    // 1. 先将数据转换成bytes
    // 2. 根据position更新slot
    pub fn add_bool(&mut self, flatbuffer: &mut FlatBuffer, position: usize, data: bool) {
        let mut value = Vec::with_capacity(2);
        let flatbuffer_data_len = match flatbuffer.data.clone() {
            None => 0,
            Some(buffer_data) => buffer_data.borrow().len(),
        };
        value.push(SEPARATOR);
        if data == false {
            value.push(0u8);
        } else {
            value.push(1u8);
        }
        let data = flatbuffer.data.clone();
        let table = flatbuffer.vtable.clone().unwrap();
        match data {
            None => flatbuffer.data = Some(Rc::new(RefCell::new(value))),
            Some(data) => data.borrow_mut().append(&mut value),
        };
        // 更新vtable
        // offset = buffer.data.len + 2
        let offset: u32 = flatbuffer_data_len as u32 + 2;
        // 定位到slot
        // let slot_start = position * 4;
        table.borrow_mut()[position] = offset;
    }
    pub fn add_string(&mut self, flatbuffer:&mut FlatBuffer, position: usize, data: String) {
        let mut data_vec = data.into_bytes();
        let mut value = Vec::with_capacity(data_vec.len() + 1);
        value.push(SEPARATOR);
        value.append(&mut data_vec);

        let data = flatbuffer.data.clone();
        let table = flatbuffer.vtable.clone().unwrap();
        let flatbuffer_data_len = match flatbuffer.data.clone() {
            None => 0,
            Some(buffer_data) => buffer_data.borrow().len(),
        };
        match data {
            None => flatbuffer.data = Some(Rc::new(RefCell::new(value))),
            Some(data) => data.borrow_mut().append(&mut value),
        };
        // 更新vtable
        // offset = buffer.data.len + 1 + child_pivot
        
        let offset: u32 = flatbuffer_data_len as u32 + 2;
        // 定位到slot
        table.borrow_mut()[position] = offset;
    }
    pub fn add_f32(){}
    pub fn add_f64(){}

    pub fn add_u8(){}
    pub fn add_u16(){}
    pub fn add_u32(){}
    pub fn add_u64(){}
    pub fn add_u128(){}
    pub fn add_usize(){}

    pub fn add_i8(){}
    pub fn add_i16(){}
    pub fn add_i32(){}
    pub fn add_i64(){}
    pub fn add_i128(){}
    pub fn add_isize(){}

    pub fn add_list<T>(&mut self, flatbuffer:&mut FlatBuffer, position: usize, data: Vec<T>) 
        where T: 'static + Clone // lifetime 啥情况
    {
        // 得到最终需要 bytes
        let list_num = data.len();
        let flatbuffer_data_len = match flatbuffer.data.clone() {
            None => 0,
            Some(buffer_data) => buffer_data.borrow().len(),
        };
        let mut flatbuffer_child = FlatBuffer::with_pivot(list_num as u8);
        // 把元素填入 flatbuffer_child 中
        for i in 0..list_num {
            let data_clone = data[i].clone();
            self.add_object(&mut flatbuffer_child, i, &data_clone);
        }
        // 转为bytes 已经有 SEPARATOR
        let (value, child_pivot) = match flatbuffer_child.bytes() {
            (None, n) => (None, n as u32),
            (Some(bytes), pivot) => (Some(bytes), pivot as u32),
        };
        let mut value = value.unwrap();
        let data = flatbuffer.data.clone();
        let table = flatbuffer.vtable.clone().unwrap();
        match data {
            None => flatbuffer.data = Some(Rc::new(RefCell::new(value))),
            Some(data) => data.borrow_mut().append(&mut value),
        };
        
        let offset: u32 = flatbuffer_data_len as u32 + 1 + child_pivot;
        // 定位到slot
        // let slot_start = position * 4;
        table.borrow_mut()[position] = offset;
    }
    // 专门为 list类型准备的
    pub fn add_object<T>(&mut self, flatbuffer:&mut FlatBuffer, position: usize, data: &T) 
        where T: Any
    {
        // 可以转化为只考虑 原始类型和非原始类型，直接取底层值
        // 如果 T 属于 String 类型
        let value_any = data as &Any;
        match value_any.downcast_ref::<String>() {
            Some(as_string) => self.add_string(flatbuffer, position, as_string.as_str().to_string()),
            None => (),
        }
        match value_any.downcast_ref::<bool>() {
            Some(as_bool) => self.add_bool(flatbuffer, position, *as_bool),
            None => (),
        }
    }
    pub fn add_flatbuffer_bytes(&mut self, 
                            flatbuffer: &mut FlatBuffer, 
                            position: usize, 
                            value:Vec<u8>, pivot: u32) 
    {
        let mut value = value;
        // 计算出 flatbuffer.data的长度
        let flatbuffer_data_len = match flatbuffer.data.clone() {
            None => 0,
            Some(buffer_data) => buffer_data.borrow().len(),
        };
        let mut offset: u32 = flatbuffer_data_len as u32 + 1 + pivot;
        if pivot == 0 {
            // 强行 + 1 可能会有大问题 0和1等效！！！！
            offset += 1;
        }
        let data = flatbuffer.data.clone();
        let table = flatbuffer.vtable.clone().unwrap();
        table.borrow_mut()[position] = offset;
        // 可能有 SEPARATOR 问题
        match data {
            None => flatbuffer.data = Some(Rc::new(RefCell::new(value))),
            Some(data) => data.borrow_mut().append(&mut value),
        };
    }

    // Vec => | field_0 | field_1 | field_2 | ... |
    // 生成一个基本的 FlatBuffer 
    // 执行循环，每一次都会得到相应字段的 child_flatbuffer
    // 将 child_flatbuffer 加入 flatbuffer 更新 vtable

    // FlatBufferVar.type 判断字段类型
    // FBString => FlatBuffer 需要 FlatBufferVar.value
    // FBBool => FlatBuffer 需要 FlatBufferVar.value
    // FBList => Flatbuffer 需要 FlatBufferVare.len FlatBufferVar.child(FlatBufferVar的数组)
    // FBStruct => Flatbuffer 需要 Flatbuffer.value 值也是bytes, 
    pub fn build(&mut self, message:&mut Vec<Option<FlatBufferVar>>, len: Option<u8>)
                 -> Result<FlatBuffer, &'static str> {
        let slot_num = len.unwrap();
        let mut flatbuffer = FlatBuffer::with_pivot(slot_num);
        for position in 0..message.len() {
            // 当vec的元素为空时，表示slot=0，无需操作
            let mut field = match message[position] {
                Some(ref mut field) => field, // 其实 ref 不懂
                None => continue,
            };
            let (t, v, l, child, p) = field.inner_field();
            match t {
                FlatBufferType::FBBool => {
                    // 直接调用 add_bool
                    match v {
                        Some(mut data) => self.add_vec(&mut flatbuffer, position, &mut data),
                        None => continue,
                    }
                },
                FlatBufferType::FBString => {
                    match v {
                        Some(mut data) => self.add_vec(&mut flatbuffer, position, &mut data),
                        None => continue,
                    }
                },
                FlatBufferType::FBList => {
                    // 生成 flatbuffer_child
                    let mut message = match child {
                        Some(child) => child,
                        None => continue,
                    };
                    // 把List中的元素，转为 可以生成 flatbuffer 的数组(Option)
                    let mut child = Vec::new();
                    for ele in message {
                        child.push(Some(ele));
                    }
                    match self.build(&mut child, l) {
                        Ok(mut flatbuffer_child) => {
                            let (list_bytes, pivot) = flatbuffer_child.bytes();
                            self.add_flatbuffer_bytes(&mut flatbuffer, position, list_bytes.unwrap(), pivot as u32);
                        },
                        Err(e) => continue,
                    }
                },
                FlatBufferType::FBStruct => {
                    // value 是子结构体的 名字，遍历 Messages 来获取message
                    match v {
                        Some(mut data) => self.add_flatbuffer_bytes(&mut flatbuffer, position, data, p.unwrap()),
                        None => continue,
                    }
                }
            };
        }
        Ok(flatbuffer)
    }
    fn add_vec(&mut self, flatbuffer: &mut FlatBuffer, position: usize, data: &mut Vec<u8>) {
        let mut value = Vec::new();
        value.push(SEPARATOR);
        value.append(data);
        self.add_flatbuffer_bytes(flatbuffer, position, value, 1u32);
    }

    // 注册实例到builder
    pub fn find_instance(&self, name: &str) -> Result<&InstanceInfo, String> {
        // 最好不要用clone() value 会很大，重复内容要尽量少
        let instance_num = self.instance_list.len();
        
        for i in 0..instance_num {
            if self.instance_list[i].instance_name.eq(name) {
                // 找到了
                return Ok(&self.instance_list[i]);
            }
        }
        return Err(format!("No this instance named {:?}", name));
    }
    pub fn from_list_bool(&mut self, mem_value: &mut Vec<bool>) -> Result<Vec<Vec<u8>>, String> {
        let mut res = Vec::new();
        let len = mem_value.len();
        for i in 0..len {
            let value = mem_value[i];
            if value == true {
                res.push(vec![1u8]);
            } else {
                res.push(vec![0u8]);
            }
        }
        Ok(res)
    }
    pub fn from_list_string(&mut self, mem_value: &mut Vec<String>) -> Result<Vec<Vec<u8>>, String> {
        let mut res = Vec::new();
        mem_value.reverse();
        let len = mem_value.len();
        for i in 0..len {
            res.push(mem_value.pop().unwrap().into_bytes());
        }
        Ok(res)
    }
    pub fn register_instance_info(  &mut self, ins_name: String, ins_type: String, 
                                    mem_info_list:&mut Vec<Option<(String, Vec<Vec<u8>>)>>) 
                                    -> Result<(), String> {
        // 根据 mem_list_info 得到结构体的字段数
        // 每一个字段都要生成 MemberInfo  (type:String Vec<值>:Vec<Vec<u8>>) => MemberInfo
        // 将 所有的Some(MemberInfo) 存在Vec中
        // 生成 InstanceInfo
        // 添加入manager
        mem_info_list.reverse();
        let member_num = mem_info_list.len();
        let mut mem_list = Vec::new();
        for i in 0..member_num {
            let mem_info = match mem_info_list.pop().unwrap() {
                    Some((mem_type, mem_value)) => Some(MemberInfo{member_type: mem_type, member_value: mem_value }),
                None => None,
            };
            mem_list.push(mem_info);
        }
        let mut instance =  InstanceInfo { 
                                instance_name: ins_name, 
                                instance_type: ins_type, 
                                member_info: mem_list 
                            };
        self.instance_list.push(instance);
        Ok(())
    }
}