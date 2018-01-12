# FlatBuffers-Rust
使用Rust实现FlatBuffers
需要手动遍历序列化的实例的每个字段
类型暂时仅支持 String bool Vec Struct 可以相互嵌套
flatbuffer 定义FlatBuffer的结构
flatbuffer_var FlatBuffer中的类型
flatbuffer_helper 原生类型和FlatBufferVar相互转化
flatbuffer_builder 创建 FlatBuffer, 增加FlatBuffer中的字段
flatbuffer_handler 解析 bytes形式的FlatBuffer
flatbuffer_manager 存储结构体字段信息（帮助解析定位）

如何更方便的将结构体转化为 Vec<FlatBufferVar>
增加对Rust原生类型转化的支持

欢迎大家指点，提意见
How to make Type V convert to Vec<T> if V if Vec<_> ?