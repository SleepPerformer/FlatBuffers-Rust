#[macro_use]
extern crate flatbuffers_rust;

use flatbuffers_rust::flatbuffers::flatbuffer_builder::FlatBufferBuilder;
use flatbuffers_rust::flatbuffers::flatbuffer::FlatBuffer;
use flatbuffers_rust::flatbuffers::flatbuffer_helper::FlatBufferHelper;
use flatbuffers_rust::flatbuffers::flatbuffer_handler::FlatBufferHandler;
use flatbuffers_rust::flatbuffers::flatbuffer_manager::*;
use std::cell::RefCell;
use std::rc::Rc;
// static SEPARATOR: u8 = 0xff;
#[derive(Debug, Clone)]
struct TestMessageChild {
    field_0: Option<String>,
    field_1: Option<String>,
    field_2: Option<bool>,
    field_3: Option<String>,
    field_4: Option<Vec<Vec<bool>>>
}
impl TestMessageChild {
    fn new() -> TestMessageChild {
        TestMessageChild {
            field_0: None,
            field_1: None,
            field_2: None,
            field_3: None,
            field_4: None
        }
    }
}

#[derive(Debug)]
struct TestMessage {
    field_0: Option<Vec<TestMessageChild>>,
    field_1: Option<String>,
    field_2: Option<bool>,
    field_3: Option<TestMessageChild>,
    field_4: Option<String>,
    field_5: Option<bool>,
    field_6: Option<bool>,
    field_7: Option<Vec<String>>,
    field_8: Option<bool>,
    field_9: Option<String>,
    field_10: Option<TestMessageChild>
}

fn init_struct_child(field_0: &str) -> TestMessageChild {
    TestMessageChild {
        field_0: Some(String::from(field_0)),
        field_1: Some(String::from("field_1")),
        field_2: Some(false),
        field_3: None,
        field_4: Some(vec![vec![true, false, false], vec![false, true]]),
    }
}
fn init_struct( child_0: TestMessageChild, 
                child_1: TestMessageChild, 
                child_2: TestMessageChild ) -> TestMessage {
    TestMessage {
        field_0: Some(vec![child_0, child_1]),
        field_1: Some(String::from("field_100")),
        field_2: None,
        field_3: None,
        field_4: None,
        field_5: None,
        field_6: Some(true),
        field_7: Some(vec![ String::from("field_7_child_0"), 
                            String::from("field_7_child_1"), 
                            String::from("field_7_child_2") ]),
        field_8: None,
        field_9: None,
        field_10: Some(child_2),
    }
}
// 返回的是flatbuffer 并不是flatbuffervar 
// var只有helper 才能生成， builder专门生成 flatbuffer
fn ser_test_child_v2(child: TestMessageChild) -> Result<FlatBuffer, &'static str> {
    let mut helper = FlatBufferHelper{};
    let mut builder = FlatBufferBuilder::new();
    let message = child;

    //  数组内容的转化
    let mut field_4 = message.field_4.unwrap();
    let mut message_field_4_vec = Vec::new();
    for element in &mut field_4 {
        message_field_4_vec.push(helper.from_vec(element, 0u8).unwrap());
    }

    let flatbuffer = build_flatbuffer_or_error! {
        {
            helper.from_string(message.field_0.unwrap()),
            helper.from_string(message.field_1.unwrap()),
            helper.from_bool(message.field_2.unwrap()),
            None,
            // 这个字段 是List<T>类型 T 为 FlatBufferVar, string, bool 一种
            // plan A 在往 field_4 内填写数据时 已经转为flatbuffervar类型了
            //        定义结构体时 直接将List<T> 定义为List<FlatBufferVar>
            // plan B 手动将内部的元素取出，全部转为FlatBufferVar
            //        添加入数组，数组参与传输
            helper.from_vec(&message_field_4_vec, 2u8),
            // helper.from_list(message.field_4.unwrap(), vec![0u8,3u8]), // 这个才是最理想的接口
        }
        builder
    };
    // println!("flatbuffer is {:?}", flatbuffer);
    flatbuffer
}
fn ser_test_message_v2(instance: TestMessage) -> Result<FlatBuffer, &'static str> {
    let mut helper = FlatBufferHelper{};
    let mut builder = FlatBufferBuilder::new();
    // 先定义一个 要转的简单的结构体类型
    let instance = instance;
    
    // 针对每一项进行转化

    // 第一项是数组
    // 针对数组的操作是
    // 将数组内的内容取出来, 每一项 -> Some(flatbuffer) 
    // Some(flatbuffer) -> flatbuffer -> Some(flatbuffervar) -> Vec<FlatBufferVar>
    let mut field_0_to_flatbuffervar_vec = Vec::new();
    let mut field_0 = instance.field_0.unwrap();
    field_0.reverse();
    let len = field_0.len();
    for i in 0..len {
        let child = field_0.pop().unwrap();
        let mut child_flatbuffer = ser_test_child_v2(child).unwrap();
        field_0_to_flatbuffervar_vec.push(helper.from_flatbuffer(&mut child_flatbuffer).unwrap());
    }
    // 第11个字段 是自定义结构体，先将结构体转为flatbuffer
    let field_10 = instance.field_10.unwrap();
    let mut field_10_flatbuffer = ser_test_child_v2(field_10).unwrap();

    let flatbuffer = build_flatbuffer_or_error! {
        {
            helper.from_vec(&field_0_to_flatbuffervar_vec, 2u8),
            helper.from_string(instance.field_1.unwrap()),
            None, None, None, None,
            helper.from_bool(instance.field_6.unwrap()),
            helper.from_vec(&instance.field_7.unwrap(), 1u8),
            None, None,
            helper.from_flatbuffer(&mut field_10_flatbuffer),
        }
        builder 
    };
    flatbuffer
}
fn ser_test_child(child: TestMessageChild) -> Result<FlatBuffer, &'static str> {
    let mut helper = FlatBufferHelper{};
    let mut builder = FlatBufferBuilder::new();
    let message = child;
    // 整个序列化过程不使用引用,
    // 需要flatbuffer化的结构体本身就是没有过多的指针类型
    // 参与转化的结构体 本身就是个临时的变量（只是值相同）
    let mut fields = Vec::new();
    push_string! {
        {
            message.field_0.unwrap(),
            message.field_1.unwrap(),
        },
        helper, fields
    }
    push_bool! {
        { message.field_2.unwrap(),},
        helper, fields
    }
    push_null! {
        {None,}, helper, fields
    }
    // 第5个字段是 Vec<Vec<bool>> 要先生成其中的内容的flatbuffer形式
    let mut field_4_childen = Vec::new();
    push_vec! {
        {
            &mut message.field_4.clone().unwrap()[0], 0u8,
            &mut message.field_4.clone().unwrap()[1], 0u8, 
        },
        helper, field_4_childen
    }
    // 这里要将field_4_child进行 (元素去Option) 算是bug吧
    let mut childen = Vec::new();
    for e in field_4_childen {
        childen.push(e.unwrap());
    }
    push_vec! {
        { &mut childen, 2u8, },
        helper, fields
    }
    builder.build(&mut fields, Some(5))
}
fn ser_test_message() -> Result<FlatBuffer, &'static str> {
    // 要使用的工具
    let mut builder = FlatBufferBuilder::new();
    let mut helper = FlatBufferHelper{};
    // 生成 临时结构体实例
    let child_0 = init_struct_child("child_0");
    let child_1 = init_struct_child("child_1");
    let child_2 = init_struct_child("child_2");
    let mut father = init_struct(child_0, child_1, child_2);

    let mut fields = Vec::new(); // 结构体所有FlatBufferVar数组
    // 第一个字段是结构体数组
    let mut field_0 = father.field_0.unwrap();
    let field_0_vec_1 = field_0.pop().unwrap();
    let field_0_vec_0 = field_0.pop().unwrap();
    
    let test_message_child_flatbuffervar_0 = helper.from_flatbuffer(&mut ser_test_child(field_0_vec_0).unwrap()).unwrap();
    let test_message_child_flatbuffervar_1 = helper.from_flatbuffer(&mut ser_test_child(field_0_vec_1).unwrap()).unwrap();
    
    push_vec! {
        { &mut vec![test_message_child_flatbuffervar_0, test_message_child_flatbuffervar_1], 2u8,},
        helper, fields
    }
    push_string! {
        { father.field_1.unwrap(), },
        helper, fields
    }
    push_null! {
        { None, None, None, None, },
        helper, fields
    }
    push_bool! {
        { father.field_6.unwrap(), },
        helper, fields
    }
    push_vec! {
        { &mut father.field_7.unwrap(), 1u8,},
        helper, fields
    }
    push_null! {
        { None, None,},
        helper, fields
    }
    let mut field_10 = ser_test_child(father.field_10.unwrap()).unwrap();
    push_flatbuffervar! {
        {field_10, },
        helper, fields
    }
    builder.build(&mut fields, Some(11))
}
fn serialize() -> (Option<Vec<u8>>, usize) {
    // 进行序列化处理
    let message = init_struct_child("message");
    let message_0 = init_struct_child("message_0");
    let message_1 = init_struct_child("message_1");
    let message_2 = init_struct_child("message_2");
    let father = init_struct(message_0, message_1, message_2);
    // 调用 build 需要 字段的 Vec<Option<FlatBufferVar>>
    let mut builder = FlatBufferBuilder::new();
    let mut helper = FlatBufferHelper{};
    // 先写死。
    // 通过 value: V type: String(通过反射成类型？) 构造 FlatBufferVar数组
    let mut fields = Vec::new(); 
    fields.push(helper.from_string(message.field_0.unwrap()));
    fields.push(helper.from_string(message.field_1.unwrap())); 
    fields.push(helper.from_bool(message.field_2.unwrap()));
    fields.push(None);
    // 针对 数组 根据类型一层一层往里找，直到元素是 原始类型
    // 其中的类型只能为 原始类型, 或 FlatBufferVar
    let mut field_4_childen = Vec::new();

    field_4_childen.push(helper.from_vec(&mut message.field_4.clone().unwrap()[0], 0u8).unwrap()); // 这里转化要注意
    field_4_childen.push(helper.from_vec(&mut message.field_4.clone().unwrap()[1], 0u8).unwrap());
    // 针对 child_flatbuffervar 生成 flatbuffer

    fields.push(helper.from_vec(&mut field_4_childen, 2u8));
    // 生成最终的flatbuffer
    let mut flatbuffer = builder.build(&mut fields, Some(5)).unwrap();
    let mut flatbuffer_0 = flatbuffer.clone();
    let mut flatbuffer_1 = flatbuffer.clone();
    let mut flatbuffer_2 = flatbuffer.clone();
    // println!("origin way : {:?}", flatbuffer);

    // 以上只是生成了 TestMessage 中的一个字段 
    // 生成一个FlatBufferVar
    let test_message_child_flatbuffervar_0 = helper.from_flatbuffer(&mut flatbuffer_0);
    // println!("test_message_child_flatbuffervar_0 {:?}", test_message_child_flatbuffervar_0);
    let test_message_child_flatbuffervar_1 = helper.from_flatbuffer(&mut flatbuffer_1);

    let mut father_fields = Vec::new();
    let mut vec = vec![test_message_child_flatbuffervar_0.unwrap(), test_message_child_flatbuffervar_1.unwrap()]; 
    
    father_fields.push(helper.from_vec(&mut vec, 2u8));

    father_fields.push(helper.from_string(father.field_1.unwrap()));
    father_fields.push(None);
    father_fields.push(None);
    father_fields.push(None);
    father_fields.push(None);
    father_fields.push(helper.from_bool(father.field_6.unwrap()));
    father_fields.push(helper.from_vec(&mut father.field_7.unwrap(), 1u8));
    father_fields.push(None);
    father_fields.push(None);
    father_fields.push(helper.from_flatbuffer(&mut flatbuffer_2));

    let mut father_flatbuffer = builder.build(&mut father_fields, Some(11)).unwrap();
    father_flatbuffer.bytes()
}
fn main() {
    let handler = FlatBufferHandler{};
    let helper = FlatBufferHelper{};
    // ser_test_child(init_struct_child("child_0"));
    // ser_test_child_v2(init_struct_child("child_0"));
    let instance = init_struct(init_struct_child("child_0"), init_struct_child("child_1"), init_struct_child("child_2"));
    let mut test_message_flatbuffer = ser_test_message_v2(instance).unwrap();
    
    // println!("最新版本输出 {:?}", test_message_flatbuffer);
    // println!("");
    // let test_message_flatbuffer_2 = ser_test_message().unwrap();
    // println!("上一版本输出 {:?}", test_message_flatbuffer_2);
    
    let mut manager = FlatBufferManager::new();

    register_struct_info! {
        struct FBTestMessageChild {
            field_0: String,
            field_1: String,
            field_2: String,
            field_3: String,
            field_4: List<List<bool>>
        }, 
        &mut manager.struct_list
    }
    register_struct_info! {
        struct FBTestMessage {
            field_0: List<TestMessageChild>,
            field_1: String,
            field_2: bool,
            field_3: TestMessageChild,
            field_4: String,
            field_5: bool,
            field_6: bool,
            field_7: List<String>,
            field_8: bool,
            field_9: String,
            field_10: TestMessageChild
        }, 
        &mut manager.struct_list
    }

    // let (data, root) = ser_test_message().unwrap().bytes();
    // let (data2, root2) = serialize();
    let (data, root) = test_message_flatbuffer.bytes();
    let data = data.unwrap();
    let want = "field_0[0].field_0";
    let mut vec = manager.to_local_vec(want, "TestMessage").unwrap();

    let mut position = manager.field_position(&mut vec).unwrap(); // 获得位置vec
    let (value_bytes, root) =  handler.get_part_data_level(&mut position, root, &data).unwrap();

    let value_bytes = value_bytes.unwrap();
    let value = helper.to_string(&value_bytes, root);
    println!("{} is {}",want, value);
}



