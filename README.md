#使用Rust实现FlatBuffers

需要手动遍历序列化的实例的每个字段，但是序列化需要更大空间

类型暂时仅支持 String， bool， Vec， Struct， 可以相互嵌套

直接定位解析数据位置，速度更快

#Structure

- flatbuffer -  Define the structure of the FlatBuffers.
- flatbuffer_var - Define some types in FlatBuffers.
- flatbuffer_helper - Realize the transform between primitive type and FlatBufferVar 
- flatbuffer_builder - Create a FlatBuffer instance, add fields to FlatBuffer
- flatbuffer_handler - Get the filed which you want in bytes
- flatbuffer_manager - Save primitive struct infomation

#Future to do

Convert  primitive type to **Vec<FlatBufferVar>** faster ?
Add more primitive type

#Issue

How to make Type **V** convert to **Vec<T>** if **V** is **Vec<_>** ?