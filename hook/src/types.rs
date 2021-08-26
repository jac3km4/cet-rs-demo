use cet_class::{CppClass, CppClassMethods};
use cet_class_derive::*;

#[repr(C)]
pub struct Handle {
    instance: *const usize,
    ref_count: *const usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CName {
    pub hash: u64,
}

#[repr(C)]
pub struct DynArray<A> {
    entries: *const A,
    cap: u32,
    size: u32,
}

impl<A> DynArray<A> {
    pub fn len(&self) -> usize {
        self.size as usize
    }

    pub fn as_slice<'a>(&'a self) -> &'a [A] {
        unsafe { std::slice::from_raw_parts(self.entries, self.size as usize) }
    }
}

#[vft_class]
pub struct ISerializable {
    handle: Handle,
    unk18: u64,
    unk20: u64,
    unk28: u64,
}

#[rustfmt::skip]
#[vft_class_impl]
impl ISerializable {
    #[virt] pub fn get_native_type(&self) -> *const RTTIType;
    #[virt] pub fn get_parent_type(&self) -> *const RTTIType;
}

#[vft_class_extending(ISerializable)]
pub struct IScriptable {
    class_type: *const usize,
    value_holder: *const usize,
}

#[vft_class_impl]
impl IScriptable {}

#[vft_class]
pub struct RTTIType {
    unk8: u64,
    pub parent: *const usize,
    pub name: CName,
    pub computed_name: CName,
}

#[rustfmt::skip]
#[vft_class_impl]
impl RTTIType {
    #[virt] pub fn sub_00(&self);
    #[virt] pub fn get_name(&self) -> CName;
    #[virt] pub fn get_size(&self) -> u32;
    #[virt] pub fn get_alignment(&self) -> u32;
    #[virt] pub fn get_type(&self) -> MetaType;
    #[virt] pub fn get_type_name(&self);
    #[virt] pub fn get_computed_name(&self) -> CName;
}

#[repr(C)]
pub struct StackFrame {
    pub code: *const u8,
    pub function: *const Function,
    pub locals: *const u8,
    pub params: *const u8,
}

#[vft_class]
pub struct Function {
    pub full_name: CName,
    pub short_name: CName,
    pub ret_prop: *const Property,
    unk20: u64,
    pub params: DynArray<*const Property>,
    pub locals: DynArray<*const Property>,
}

#[repr(C)]
pub struct Property {
    pub typ: *const RTTIType,
    pub name: CName,
}

#[repr(u8)]
#[derive(Debug)]
pub enum MetaType {
    Name = 0,
    Fundamental = 1,
    Class = 2,
    Array = 3,
    Simple = 4,
    Enum = 5,
    StaticArray = 6,
    NativeArray = 7,
    Pointer = 8,
    Handle = 9,
    WeakHandle = 10,
    ResourceReference = 11,
    ResourceAsyncReference = 12,
    BitField = 13,
    LegacySingleChannelCurve = 14,
    ScriptReference = 15,
}
