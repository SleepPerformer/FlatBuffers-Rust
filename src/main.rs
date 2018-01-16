#[macro_use]
extern crate flatbuffers_rust;

use flatbuffers_rust::flatbuffers::flatbuffer_builder::FlatBufferBuilder;
use flatbuffers_rust::flatbuffers::flatbuffer::FlatBuffer;
use flatbuffers_rust::flatbuffers::flatbuffer_helper::FlatBufferHelper;
use flatbuffers_rust::flatbuffers::flatbuffer_handler::FlatBufferHandler;
use flatbuffers_rust::flatbuffers::flatbuffer_manager::*;

macro_rules! deser_difftime {
    () => {
    let start = time::now();//获取开始时间

    //  反序列化函数

    let end = time::now();//获取结束时间
    println!("done!start : {:?},end :{:?},duration:{:?}",start,end,end-start);
    };
}

macro_rules! ser_difftime {
    () => {
    let start = time::now();//获取开始时间

    //  序列化函数

    let end = time::now();//获取结束时间
    println!("done!start : {:?},end :{:?},duration:{:?}",start,end,end-start);
    };
}
// 第一组测试类型
#[derive(Debug, Clone, PartialEq)]
struct TestMessage_0 {
    field_0: Option<Vec<TestMessageChild_1>>,
    field_1: Option<String>,
    field_2: Option<bool>,
    field_3: Option<TestMessageChild_1>,
    field_4: Option<String>,
    field_5: Option<bool>,
    field_6: Option<TestMessageChild_2>,
    field_7: Option<Vec<String>>,
    field_8: Option<bool>,
    field_9: Option<String>,
    field_10: Option<TestMessageChild_0>
}
impl TestMessage_0 {
    fn new( field_0: Vec<TestMessageChild_1>,
            field_4: &str,
            field_6: Option<TestMessageChild_2>,
            field_7: Option<Vec<String>>,
            field_10: Option<TestMessageChild_0>)
             -> TestMessage_0 {
        TestMessage_0 {
            field_0: Some(field_0),
            field_1: Some(String::from("field_100")),
            field_2: None,
            field_3: None,
            field_4: Some(String::from(field_4)),
            field_5: Some(false),
            field_6: field_6,
            field_7:field_7,
            field_8: Some(true),
            field_9: None,
            field_10:field_10  
        }
    }
    fn with_none() -> TestMessage_0 {
        TestMessage_0 {
            field_0: None,
            field_1: None,
            field_2: None,
            field_3: None,
            field_4: None,
            field_5: None,
            field_6: None,
            field_7: None,
            field_8: None,
            field_9: None,
            field_10: None
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct TestMessageChild_0 {
    field_0: Option<String>,
    field_1: Option<String>,
    field_2: Option<bool>,
    field_3: Option<TestMessageChild_2>,
    field_4: Option<Vec<Vec<bool>>>
}
impl TestMessageChild_0 {
    fn new( field_0: &str,
            field_3: TestMessageChild_2)
             -> TestMessageChild_0 {
        TestMessageChild_0 {
            field_0: Some(String::from(field_0)),
            field_1: Some(String::from("TestMessageChild_0")),
            field_2: Some(false),
            field_3: Some(field_3),
            field_4: Some(vec![vec![true, false, false], vec![false, true]])
        }
    }
    fn with_none() -> TestMessageChild_0 {
        TestMessageChild_0 {
            field_0: None,
            field_1: None,
            field_2: None,
            field_3: None,
            field_4: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct TestMessageChild_1 {
    field_0: Option<String>,
    field_1: Option<String>,
    field_2: Option<bool>,
    field_3: Option<String>
}
impl TestMessageChild_1 {
    fn new(field_2: Option<bool>) -> TestMessageChild_1 {
        TestMessageChild_1 {
            field_0: Some(String::from("field_0")),
            field_1: Some(String::from("TestMessageChild_1")),
            field_2: field_2,
            field_3: None
        }
    }
    fn with_none() -> TestMessageChild_1 {
        TestMessageChild_1 {
            field_0: None,
            field_1: None,
            field_2: None,
            field_3: None
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct TestMessageChild_2 {
    field_0: Option<String>,
    field_1: Option<bool>,
    field_2: Option<TestMessageChild_1>
}
impl TestMessageChild_2 {
    fn new() -> TestMessageChild_2 {
        TestMessageChild_2 {
            field_0: Some(String::from("TestMessageChild_2")),
            field_1: Some(true),
            field_2: None
        }
    }
}
// 生成实例 根据测试需要进行修改
fn init_child_2() -> TestMessageChild_2 {
    TestMessageChild_2::new()
}
fn init_child_0(message: &str) -> TestMessageChild_0 {
    TestMessageChild_0::new(message, init_child_2())
}
fn init_child_1(bool_value: Option<bool>) -> TestMessageChild_1 {
    TestMessageChild_1::new(bool_value)
}
fn init_father( field_0: Vec<TestMessageChild_1>,
                field_4: &str,
                field_6: Option<TestMessageChild_2>,
                field_7: Option<&mut Vec<&str>>,
                field_10: Option<TestMessageChild_0> )
                 -> TestMessage_0
    {
        let string_vec = match field_7 {
            Some(field_7) => {
                let mut string_vec = Vec::new();
                let len = field_7.len();
                for i in 0..len {
                    let string = field_7.pop().unwrap();
                    string_vec.push(String::from(string));
                }
                string_vec.reverse();
                Some(string_vec)
            },
            None => None,
        };
        TestMessage_0::new(field_0, field_4, field_6, string_vec, field_10)
}

// 生成 father instance
fn father_instance() -> TestMessage_0 {
    let child_1_0 = init_child_1(Some(false));
    let child_1_1 = init_child_1(None);
    let child_1_2 = init_child_1(Some(true));
    let mut field_0 = vec![child_1_0, child_1_1, child_1_2];
    let mut field_7 = vec!["Use", "buffers", "in", "Rust"];
    let father = init_father(field_0, "Father", Some(TestMessageChild_2::new()), Some(&mut field_7), Some(init_child_0("Last field")));
    father
}
/**
 * 每一个字段都是Some(T)或None, Some 不显示了
 * child_0 
 * {
        field_0: "Last field",
        field_1: "TestMessageChild_0",
        field_2: false,
        field_3: child_2_1,
        field_4: vec![ vec![true, false, false], vec![false, true] ]   
 * }
 * 
 * child_1_0                                     * child_1_1                                     * child_1_2 
 * {                                             * {                                             * {
        field_0: "field_0",                             field_0: "field_0",                             field_0: "field_0",
        field_1: "TestMessageChild_1",                  field_1: "TestMessageChild_1",                  field_1: "TestMessageChild_1",
        field_2: false,                                 field_2: false,                                 field_2: true,
        field_3: None                                   field_3: None                                   field_3: None
 * }                                             * }                                             * }

 * child_2_0                                     * child_2_1 
 * {                                             * {
        field_0: "TestMessageChild_2",                  field_0: "TestMessageChild_2",
        field_1: true,                                  field_1: true,
        field_2: None                                   field_2: None 
 * }                                             * }

 * father instance
 * {
        field_0 : vec![child_1_0, child_1_1, child_1_2],
        field_1 : "field_100"
        field_2 : None,
        field_3 : None,
        field_4 : "Father",
        field_5 : false,
        field_6 : child_2_0,
        field_7 : vec!["Use", "buffers", "in", "Rust"],
        field_8 : true,
        field_9 : None, 
        field_10 : child_0
 * }
 */
fn serialization_father() -> (Option<Vec<u8>>, usize){
    // 要使用的工具
    let mut builder = FlatBufferBuilder::new();
    let mut helper = FlatBufferHelper{};

    let mut father = father_instance();
    // 对father 每一个字段 进行序列化
    // 第1个字段特殊处理
    let mut field_0 = match father.field_0 {
        Some(mut value) => {
            let mut field_0 = Vec::new();
            let len = value.len();
            for _i in 0..len {
                let element = value.pop().unwrap();
                let mut flatbuffer = serialization_child_1(element);
                field_0.push(helper.from_option_flatbuffer(flatbuffer).unwrap());
            }
            field_0.reverse();
            Some(field_0)

        },
        None => None,
    }; 
    //第4个字段特殊处理
    // let field_3 = serialization_child_1(field_3).unwrap();
    // 第7个字段特殊处理
    let field_6_value = father.field_6.unwrap();
    let mut field_6 = serialization_child_2(field_6_value);
    // // 第8个字段个数处理
    // let field_7 = Some(& father.field_7);
    // println!("field_7 => bytes is {:?}", builder.build(helper.from_vec(&mut father.field_7.clone().unwrap(), 1u8),Some(4u8)));
    // 第11字段特殊处理
    let field_10_value = father.field_10.unwrap();
    let mut field_10 = serialization_child_0(field_10_value);
    let final_flatbuffer = build_flatbuffer_or_error! {
        {
            helper.from_option_vec(field_0, 2u8),
            helper.from_option_string(father.field_1),
            helper.from_option_bool(father.field_2),
            // helper.from_option_flatbuffer(father.field_3), // 这里其实不太合理 不知道field_3是None 需要将其转化在调用
            None,
            helper.from_option_string(father.field_4),
            helper.from_option_bool(father.field_5),
            helper.from_option_flatbuffer(field_6),
            helper.from_option_vec(father.field_7, 1u8),
            helper.from_option_bool(father.field_8),
            helper.from_option_string(father.field_9),
            helper.from_option_flatbuffer(field_10),
        }
        builder
    };
    final_flatbuffer.unwrap().bytes()
}
fn serialization_child_0(child: TestMessageChild_0) -> Option<FlatBuffer> {
    let mut helper = FlatBufferHelper{};
    let mut builder = FlatBufferBuilder::new();
    // 第4个字段特殊处理
    let field_3_value = child.field_3.unwrap();
    let mut field_3 = Some(serialization_child_2(field_3_value).unwrap());
    // println!("child_0 child_field_3_vec is {:?}", field_3);
    // 第5个字段特殊处理
    let mut field_4 = child.field_4.unwrap();
    let mut child_field_4_vec = Vec::new();
    for element in &mut field_4 {
        // println!("element is {:?}", element);
        child_field_4_vec.push(helper.from_vec(element, 0u8).unwrap());
    }
    let child_field_4_vec = Some(child_field_4_vec);
    // println!("child_0 child_field_4_vec is {:?}", helper.from_vec(&mut field_4, 2u8));
    // 生成 flatbuffer
    let final_flatbuffer = build_flatbuffer_or_error! {
        {
            helper.from_option_string(child.field_0),
            helper.from_option_string(child.field_1),
            helper.from_option_bool(child.field_2),
            helper.from_option_flatbuffer(field_3),
            helper.from_option_vec(child_field_4_vec, 2u8),
        }
        builder
    };
    Some(final_flatbuffer.unwrap())
}

fn serialization_child_1(child: TestMessageChild_1) -> Option<FlatBuffer> {
    // 要使用的工具
    let handler = FlatBufferHandler{};

    let mut builder = FlatBufferBuilder::new();
    let mut helper = FlatBufferHelper{};

    let final_flatbuffer = build_flatbuffer_or_error! {
        {
            helper.from_option_string(child.field_0),
            helper.from_option_string(child.field_1),
            helper.from_option_bool(child.field_2),
            helper.from_option_string(child.field_3),
        }
        builder
    };
    Some(final_flatbuffer.unwrap())
}

fn serialization_child_2(child: TestMessageChild_2) -> Option<FlatBuffer> {
    let handler = FlatBufferHandler{};
    let helper = FlatBufferHelper{};
    let mut builder = FlatBufferBuilder::new();
    let final_flatbuffer = build_flatbuffer_or_error! {
        {
            helper.from_option_string(child.field_0),
            helper.from_option_bool(child.field_1),
            None, // child_1类型
        }
        builder
    };
    Some(final_flatbuffer.unwrap())
}
fn trans_bytes(bytes:&mut Vec<u8>, pivot: usize) -> Vec<u8> {
    // 将 两者 合并为 1(flag) + 1(pivot) + n(bytes) 的 形式
    let mut vec = Vec::new();
    vec.push(pivot as u8);
    vec.append(bytes);
    vec
}
fn register(manager: &mut FlatBufferManager) {
    register_struct_info! {
        struct FBTestMessageChild_0 {
            field_0: String,
            field_1: String,
            field_2: bool,
            field_3: TestMessageChild_2,
            field_4: List<List<bool>>
        },
        manager.struct_list
    }
    register_struct_info! {
        struct FBTestMessageChild_1 {
            field_0: String,
            field_1: String,
            field_2: bool,
            field_3: String
        }, 
        manager.struct_list
    }
    register_struct_info! {
        struct FBTestMessageChild_2 {
            field_0: String,
            field_1: bool,
            field_2: TestMessageChild_1
        }, 
        manager.struct_list
    }
    register_struct_info! {
        struct FBTestMessage_0 {
            field_0: List<TestMessageChild_1>,
            field_1: String,
            field_2: bool,
            field_3: TestMessageChild_1,
            field_4: String,
            field_5: bool,
            field_6: TestMessageChild_2,
            field_7: List<String>,
            field_8: bool,
            field_9: String,
            field_10: TestMessageChild_0
        }, 
        manager.struct_list
    }
}
// 反序列化过程  已经将 bytes和pivot分离
fn deserialization_father(data: &mut Vec<u8>, pivot: usize, manager: &mut FlatBufferManager) -> TestMessage_0 {
    let handler = FlatBufferHandler{};
    let helper = FlatBufferHelper{};

    let mut father = TestMessage_0::with_none();

    let mut field_0 = Vec::new();
    let want = vec!["field_0[0]", "field_0[1]", "field_0[2]"];
    for i in 0..3 {
        // let want = format!("field_0[{}]", i).as_str();
        let mut local_vec = match manager.to_local_vec(want[i], "TestMessage_0") {
            Ok(vec) => vec,
            Err(e) => panic!("local_vec {}", e),
        };
        let mut position = match manager.field_position(&mut local_vec){
            Ok(p) => p,
            Err(e) => panic!("position {}", e),
        };
        // println!("从这里开始崩溃");
        let (value_bytes, root) = match handler.get_part_data_level(&mut position, pivot, &data) {
            Ok((value_bytes, root)) => (value_bytes, root),
            Err(e) => panic!("(value_bytes, root) panic {}", e),
        };
        // value_bytes 是child类型，继续解析 
        let mut value = match value_bytes {
            Some(v) => v,
            None => panic!("value is none"),
        };
        // println!("从这里开始崩溃");
        let child_1 = deserialization_child_1(&mut value, root, manager);
        field_0.push(child_1);
    }
    father.field_0 = Some(field_0);

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_1", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    father.field_1 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_2", manager, data, pivot).unwrap();
    let value = helper.to_option_bool(value_bytes, root);
    father.field_2 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_2", manager, data, pivot).unwrap();
    let mut value = match value_bytes{
        Some(mut v) => Some(deserialization_child_1(&mut v, root, manager)),
        None => None,
    };
    father.field_3 = value; 

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_4", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    father.field_4 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_5", manager, data, pivot).unwrap();
    let value = helper.to_option_bool(value_bytes, root);
    father.field_5 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_6", manager, data, pivot).unwrap();
    let mut value = value_bytes.unwrap();
    let field_6 = deserialization_child_2(&mut value, root, manager);
    father.field_6 = Some(field_6);

    let mut field_7 = Vec::new(); 
    let want = vec!["field_7[0]", "field_7[1]", "field_7[2]", "field_7[3]"];
    for i in 0..4 {
        let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", want[i], manager, data, pivot).unwrap();
        let value_bytes = value_bytes.unwrap();
        let value = helper.to_string(&value_bytes, root);
        field_7.push(value);
    }
    father.field_7 = Some(field_7);

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_8", manager, data, pivot).unwrap();
    let value = helper.to_option_bool(value_bytes, root);
    father.field_8 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_9", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    father.field_9 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessage_0", "field_10", manager, data, pivot).unwrap();
    let mut value = value_bytes.unwrap();
    let field_10 = deserialization_child_0(&mut value, root, manager);
    father.field_10 = Some(field_10);

    father
}
fn deserialization_child_0(data: &mut Vec<u8>, pivot: usize, manager: &mut FlatBufferManager) -> TestMessageChild_0 {
    let handler = FlatBufferHandler{};
    let helper = FlatBufferHelper{};
    let mut child_0 = TestMessageChild_0::with_none();

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_0", "field_0", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    child_0.field_0 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_0", "field_1", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    child_0.field_1 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_0", "field_2", manager, data, pivot).unwrap();
    let value = helper.to_option_bool(value_bytes, root);
    child_0.field_2 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_0", "field_3", manager, data, pivot).unwrap();
    let mut value = value_bytes.unwrap();
    let field_3 = deserialization_child_2(&mut value, root, manager);
    child_0.field_3 = Some(field_3);

    let mut field_4 = Vec::new(); 
    let want = vec!["field_4[0]", "field_4[1]"];
    for i in 0..2 {
        let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_0", want[i], manager, data, pivot).unwrap();
        let value_bytes = value_bytes.unwrap();
        
        let mut value = helper.to_vec(&value_bytes, root).unwrap();
        let mut child_child = Vec::new();
        for element in &mut value {
            child_child.push(helper.to_bool(element, 1));
        }
        field_4.push(child_child);
    }
    child_0.field_4 = Some(field_4);


    child_0
}
fn deserialization_child_1(data: &mut Vec<u8>, pivot: usize, manager: &mut FlatBufferManager) -> TestMessageChild_1 {
    let handler = FlatBufferHandler{};
    let helper = FlatBufferHelper{};
    
    let mut child = TestMessageChild_1::with_none();

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_1", "field_0", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    child.field_0 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_1", "field_1", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    child.field_1 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_1", "field_2", manager, data, pivot).unwrap();
    let value = helper.to_option_bool(value_bytes, root);
    child.field_2 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_1", "field_3", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    child.field_3 = value;

    child
}

fn deserialization_child_2(data: &mut Vec<u8>, pivot: usize, manager: &mut FlatBufferManager) -> TestMessageChild_2 {
    let handler = FlatBufferHandler{};
    let helper = FlatBufferHelper{};
    let mut child_2 = TestMessageChild_2 {
        field_0: None,
        field_1: None,
        field_2: None
    };
    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_2", "field_0", manager, data, pivot).unwrap();
    let value = helper.to_option_string(value_bytes, root);
    child_2.field_0 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_2", "field_1", manager, data, pivot).unwrap();
    let value = helper.to_option_bool(value_bytes, root);
    child_2.field_1 = value;

    let (value_bytes, root) = handler.get_field_bytes_pivot("TestMessageChild_2", "field_2", manager, data, pivot).unwrap();
    let mut value = match value_bytes {
        Some(mut v) => {
            let field_2 = deserialization_child_1(&mut v, root, manager);
            child_2.field_2 = Some(field_2);
        },
        None => child_2.field_2 = None,
    };

    child_2
}

fn main() {
    let mut ser_father = father_instance();
    let (bytes, pivot) = serialization_father(); // 序列化其实已经完成
    let mut bytes =  bytes.unwrap();
    println!("ser flatbuffer bytes len is {:?}", bytes.len());
    let mut vec = trans_bytes(&mut bytes, pivot); // 参与传输的格式
    println!("trans is {:?}", vec);
    // 反序列化
    vec.reverse();
    let pivot = vec.pop().unwrap() as usize;
    vec.reverse();
    // 反序列化 转变成为 data+pivot格式
    // 尝试使用 to_struct格式
    let helper = FlatBufferHelper{};

    let mut manager = FlatBufferManager::new();
    register(&mut manager);
    // 传输过来的内容
    let deser_father = deserialization_father(&mut vec, pivot, &mut manager);
    if deser_father == ser_father {
        println!("解析成功");
    }
    println!("deser flatbuffer bytes len is {:?}", vec.len());
}



