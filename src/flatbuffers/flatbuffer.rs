use std::cell::RefCell;
use std::rc::Rc;
// flatbuffer 作为网络中传输的数据序列化工具
// 本质就是一个连续的bytes数组

/**                FlatBuffer 结构示意图  
 *       
 * SEPARATOR         vtable         pivot  SEPARATOR+data0  SEPARATOR+data1   S+d*
 *            slot0   slot1  slot*  
 *   0x55   | 4bytes  4bytes  ~~~ | 1byte | 0x55 + n bytes | 0x55 + n bytes | ~~~ | 
 *           offset from the pivot
 */

// 需要生成用于网络传输的bytes数组时，将vtable pivot data 拼接起来
static SEPARATOR: u8 = 0xff;

#[derive(Debug, Clone)]
// pivot 或者 data 至少一个不为None
pub struct FlatBuffer {
    pub vtable: Option<Rc<RefCell<Vec<u32>>>>, // 一个slot 占用的大小为4bytes
    pivot: Option<u8>, // 表示vtable的length (255应该足够) 
    pub data: Option<Rc<RefCell<Vec<u8>>>> // 原始的数据 头部填充 pivot*4+1+1 bytes
}

impl FlatBuffer {
    pub fn new() -> FlatBuffer {
        FlatBuffer {
            vtable: None,
            pivot: None,
            data: None
        }
    }
    pub fn with_pivot(num: u8) -> FlatBuffer {
        let mut vtable = Vec::with_capacity(num as usize);
        for _i in 0..num {
            vtable.push(0u32);
        }
        FlatBuffer {
            vtable: Some(Rc::new(RefCell::new(vtable))),
            pivot: Some(num),
            data: None,
        }
    }
    pub fn with_primitive_type(data:&mut Vec<u8>) -> FlatBuffer {
        // 在前方添加一个 SEPARATOR
        let len = data.len();
        let mut flatdata = Vec::with_capacity(len + 1);
        flatdata.push(SEPARATOR);
        for i in 0..len {
            flatdata.push(data[i]);
        }
        FlatBuffer {
            vtable: None,
            pivot: None,
            data: Some(Rc::new(RefCell::new(flatdata))),
        }
    }
    // 返回字节数组，并且包含root的位置
    pub fn bytes(&mut self) -> (Option<Vec<u8>>, usize) {
        // 第一个字节一定是 SEPARATOR
        let mut bytes = Vec::new();
        let mut root = 0; // (None, 0)
        // 判断是否为 primitive type 可以优化，步骤多余
        match self.pivot {
            None => (),
            Some(pivot) => { bytes.push(SEPARATOR); root = 1;},
        };
        // 将 vtable 添加入bytes中
        match self.vtable_to_bytes() {
            None => (),
            Some(mut vec) => bytes.append(&mut vec),
        };
        // 将 pivot 添加入bytes中
        match self.pivot {
            None => (),
            Some(pivot) => { bytes.push(pivot); root = bytes.len() - 1;},
        };
        // 将 data 添加入bytes中
        match self.data {
            None => (),
            // 逻辑有点问题，可能需要修改，没有clone 会出现难以想象的错误
            Some(ref mut data) => bytes.append(&mut data.borrow_mut().clone()),
        };
        if bytes.len() == 0 {
            (None, root)
        } else {
            (Some(bytes), root)
        }
    }
    // pub fn add_data(flatbuffer: FlatBuffer, add:&mut Vec<u8>) -> FlatBuffer {
    //     flatbuffer.data.unwrap().append(add);
    //     flatbuffer
    // }
    fn vtable_to_bytes(&mut self) -> Option<Vec<u8>> {
        let vtable = self.vtable.clone();
        let table = match vtable {
            None => return None,
            Some(table) => table,
        };
        let mut bytes = Vec::new();
        let slots_num = table.borrow().len();
        for i in 0..slots_num {
            let offset = table.borrow_mut()[i];
            let b0 : u8 = ((offset >> 24) & 0xff) as u8;
            let b1 : u8 = ((offset >> 16) & 0xff) as u8;
            let b2 : u8 = ((offset >> 8) & 0xff) as u8;
            let b3 : u8 = (offset & 0xff) as u8;
            bytes.push(b3);
            bytes.push(b2);
            bytes.push(b1);
            bytes.push(b0);
        }
        Some(bytes)
    }
}