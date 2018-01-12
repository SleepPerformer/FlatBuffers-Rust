static SEPARATOR: u8 = 0xff;
#[derive(Debug)]
// 在handler中做限制，防止人为访问内存出错
pub struct FlatBufferHandler {}

impl FlatBufferHandler {
    // 方法名不舒服
    // 从某一层获取到任意深层的任一数据
    // level = vec[4,2,3,1] 
    // 表示 当前“struct” 的第一个字段的 第3个字段的 第2个字段的 第4个字段
    pub fn get_part_data_level(&self, level:&mut Vec<usize>, root: usize, data: &Vec<u8>) -> Result<(Option<Vec<u8>>, usize), String> {
        let l = level.len();
        let mut want_vec = Vec::new();
        let mut want_root = 0;
        let part = level.pop().unwrap();
        let (child_data, child_root) = match self.get_part_data(part, root, &data) {
            Ok((child_data, child_root)) => (child_data, child_root),
            Err(e) => return Err(e),
        };
        match child_data {
            Some(d) => { want_vec = d; want_root = child_root; },
            None => {
                if 1 == l {
                    return Ok((None, 0));
                } else {
                    return Err(format!("访问层次有误"));
                }
            },
        };

        for i in 0..l-1 {
            let part = level.pop().unwrap();
            let (child_data, child_root) = match self.get_part_data(part, want_root, &want_vec) {
                Ok((child_data, child_root)) => (child_data, child_root),
                Err(e) => return Err(e),
            };
            match child_data {
                Some(d) => { want_vec = d; want_root = child_root; },
                None => {
                    if i == l {
                        return Ok((None, 0));
                    } else {
                        return Err(format!("访问层次有误"));
                    }
                },
            };
        }
        Ok((Some(want_vec), want_root))
    }
    pub fn get_part_data(&self, part: usize, root: usize, data: &Vec<u8>) -> Result<(Option<Vec<u8>>, usize), String> {
        // 先通过 root 的值 获取字段总数
        if part == 0 {
            return Err(format!("part is {}", part));
        }
        if root == 1 {
            // 这是个基本数据
            return Err(format!("{:?} is a primitive type", data));
        }
        let slot_num = data[root] as usize;
        if part > slot_num {
            // 越界
            return Err(format!("slot's num is {}, {} > slot's", slot_num, part));
        }
        // println!("root is {}, root's value is {}", root, data[root]);
        // 得到偏移量 [root-(slot_num-part+1)*size, root-(slot_num-part)*size]
        let (start, child_root) = self.get_start(part, slot_num, root, data);
        // println!("part2 start:{}, child root value is {}", start, data[child_root]);
        if start == 0 {
            // 该项数据为空
            return Ok((None, 0));
        }
        // 计算 end 
        let end = self.get_end(part, slot_num, root, data);
        // println!("end is {}", end);
        Ok((Some(data[start..end].to_vec()), child_root-start))
    }
    fn get_end(&self, part: usize, slot_num: usize, root: usize, data: &Vec<u8>) -> usize {
        let mut part = part;
        if slot_num == part {
            return data.len()
        } else {
            loop {
                part += 1;
                let (end, root) = self.get_start(part, slot_num, root, data);
                // println!("第8字段开始值可能为{}", end);
                if end != 0 {
                    return end;
                }
            }
        }
    }
    fn get_start(&self, part: usize, slot_num: usize, root: usize, data: &Vec<u8>) -> (usize, usize) {
        let size = 4;
        let mut offset = 0;
        let mut scale = 1;
        let offset_vec = data[root-(slot_num-part+1)*size..root-(slot_num-part)*size].to_vec();
        for i in 0..size {
            // println!("{:?}", offset_vec[i]);
            offset += (offset_vec[i] as usize) *scale;
            scale *= 256;
        }
        // println!("offset is {:?}", offset);
        if offset == 0 {
            // 偏移为0 表示 None
            return (0, 0);
        }
        let child_root = offset as usize + root;
        // println!("root is {}, offset is {}, child_root is {}", root, offset, child_root);
        // println!("root's value is {}, child_root's value is {}", data[root], data[child_root]);
        let mut start = 0;
        // 判断是否有 vtable
        if data[child_root - 1] == SEPARATOR {
            // 表示child_root是start
            start = child_root - 1;
        } else {
            // 获取到 child_vtable 大小
            let child_slot_num = data[child_root];
            start = child_root - size*child_slot_num as usize -1; // SEPARATOR 也要包括进去
        }
        (start, child_root)
    }
}