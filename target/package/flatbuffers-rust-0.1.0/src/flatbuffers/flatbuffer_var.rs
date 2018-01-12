#[derive(Debug, Clone)]
pub enum FlatBufferType {
    FBBool,
    FBString,
    FBList,
    FBStruct,
}

    // FlatBufferVar.type 判断字段类型
    // FBString => FlatBuffer 需要 FlatBufferVar.value
    // FBBool => FlatBuffer 需要 FlatBufferVar.value
    // FBList => Flatbuffer 需要 FlatBufferVar.len FlatBufferVar.child(FlatBufferVar的数组)
    // FBStruct => Flatbuffer 需要 FlatbufferVar.value 
#[derive(Debug, Clone)]
pub struct FlatBufferVar {
    t: FlatBufferType,
    v: Option<Vec<u8>>,
    l: Option<u8>,
    child: Option<Vec<FlatBufferVar>>, // list成员
    p: Option<u32>
}
impl FlatBufferVar {
    pub fn new( t: FlatBufferType, 
                v: Option<Vec<u8>>, 
                l: Option<u8>, 
                child: Option<Vec<FlatBufferVar>>,
                p: Option<u32> )
             -> Option<FlatBufferVar>
        {
            Some(FlatBufferVar { t: t, v: v, l: l, child: child, p:p })
        }
    pub fn inner_field(&mut self)
            -> (FlatBufferType, Option<Vec<u8>>, Option<u8>, Option<Vec<FlatBufferVar>>, Option<u32>)
    {
        (self.t.clone(), self.v.clone(), self.l, self.child.clone(), self.p)
    }
}
