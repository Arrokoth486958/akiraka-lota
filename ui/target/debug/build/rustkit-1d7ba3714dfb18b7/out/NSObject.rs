#[allow(unused_imports)]
use objc::*;
use Foundation::NSString;
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_performSelector_: SelectorRef =
    SelectorRef(&b"performSelector:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_isSubclassOfClass_: SelectorRef =
    SelectorRef(&b"isSubclassOfClass:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_resolveInstanceMethod_: SelectorRef =
    SelectorRef(&b"resolveInstanceMethod:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_doesNotRecognizeSelector_: SelectorRef =
    SelectorRef(&b"doesNotRecognizeSelector:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_forwardingTargetForSelector_: SelectorRef =
    SelectorRef(&b"forwardingTargetForSelector:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_isMemberOfClass_: SelectorRef =
    SelectorRef(&b"isMemberOfClass:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_hash: SelectorRef = SelectorRef(&b"hash\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_performSelector_withObject_: SelectorRef =
    SelectorRef(&b"performSelector:withObject:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_init: SelectorRef = SelectorRef(&b"init\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_resolveClassMethod_: SelectorRef =
    SelectorRef(&b"resolveClassMethod:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_instanceMethodForSelector_: SelectorRef =
    SelectorRef(&b"instanceMethodForSelector:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_description: SelectorRef = SelectorRef(&b"description\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_conformsToProtocol_: SelectorRef =
    SelectorRef(&b"conformsToProtocol:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_mutableCopy: SelectorRef = SelectorRef(&b"mutableCopy\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_self: SelectorRef = SelectorRef(&b"self\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_initialize: SelectorRef = SelectorRef(&b"initialize\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_finalize: SelectorRef = SelectorRef(&b"finalize\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_debugDescription: SelectorRef =
    SelectorRef(&b"debugDescription\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_instancesRespondToSelector_: SelectorRef =
    SelectorRef(&b"instancesRespondToSelector:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_copy: SelectorRef = SelectorRef(&b"copy\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_isProxy: SelectorRef = SelectorRef(&b"isProxy\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_superclass: SelectorRef = SelectorRef(&b"superclass\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_load: SelectorRef = SelectorRef(&b"load\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_methodForSelector_: SelectorRef =
    SelectorRef(&b"methodForSelector:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_performSelector_withObject_withObject_: SelectorRef =
    SelectorRef(&b"performSelector:withObject:withObject:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_isKindOfClass_: SelectorRef = SelectorRef(&b"isKindOfClass:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_respondsToSelector_: SelectorRef =
    SelectorRef(&b"respondsToSelector:\0"[0] as *const u8);
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_selrefs"]
pub static mut SEL_isEqual_: SelectorRef = SelectorRef(&b"isEqual:\0"[0] as *const u8);
pub trait NSObjectProto: ObjCClass {
    fn self_(&self) -> Option<Arc<Self>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> *mut Self =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(self as *const Self as *mut Self as *mut _, SEL_self);
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    fn conformsToProtocol_(&self, aProtocol: Option<&Protocol>) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, *mut Protocol) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_conformsToProtocol_,
                aProtocol
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            _ret
        }
    }
    fn respondsToSelector_(&self, aSelector: SelectorRef) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, SelectorRef) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_respondsToSelector_,
                aSelector,
            );
            _ret
        }
    }
    fn isMemberOfClass_(&self, aClass: Option<&Class>) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, *mut Class) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_isMemberOfClass_,
                aClass
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            _ret
        }
    }
    fn performSelector_withObject_withObject_(
        &self,
        aSelector: SelectorRef,
        object1: Option<&Object>,
        object2: Option<&Object>,
    ) -> Option<Arc<Object>> {
        unsafe {
            let send: unsafe extern "C" fn(
                *mut Object,
                SelectorRef,
                SelectorRef,
                *mut Object,
                *mut Object,
            ) -> *mut Object = mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_performSelector_withObject_withObject_,
                aSelector,
                object1
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
                object2
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    fn isProxy(&self) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(self as *const Self as *mut Self as *mut _, SEL_isProxy);
            _ret
        }
    }
    fn isKindOfClass_(&self, aClass: Option<&Class>) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, *mut Class) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_isKindOfClass_,
                aClass
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            _ret
        }
    }
    fn performSelector_withObject_(
        &self,
        aSelector: SelectorRef,
        object: Option<&Object>,
    ) -> Option<Arc<Object>> {
        unsafe {
            let send: unsafe extern "C" fn(
                *mut Object,
                SelectorRef,
                SelectorRef,
                *mut Object,
            ) -> *mut Object = mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_performSelector_withObject_,
                aSelector,
                object
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    fn isEqual_(&self, object: Option<&Object>) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, *mut Object) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_isEqual_,
                object
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            _ret
        }
    }
    fn performSelector_(&self, aSelector: SelectorRef) -> Option<Arc<Object>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, SelectorRef) -> *mut Object =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_performSelector_,
                aSelector,
            );
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
}
#[repr(C)]
pub struct _NSZone {
    opaque: u32,
}
extern "C" {
    #[link_name = "OBJC_CLASS_$_NSObject"]
    static NSObjectClass: Class;
}
#[allow(non_upper_case_globals)]
#[link_section = "__DATA,__objc_classrefs"]
static CLASS_NSObject: ClassRef = ClassRef(unsafe { &NSObjectClass } as *const _);
#[repr(C)]
pub struct NSObject {
    isa: *const Class,
}
impl ObjCClass for NSObject {
    fn classref() -> ClassRef {
        CLASS_NSObject
    }
}
impl NSObjectProto for NSObject {}
impl NSObject {
    pub fn load() -> () {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> () =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_load,
            );
            _ret
        }
    }
    pub fn resolveInstanceMethod_(sel: SelectorRef) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, SelectorRef) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_resolveInstanceMethod_,
                sel,
            );
            _ret
        }
    }
    pub fn initialize() -> () {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> () =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_initialize,
            );
            _ret
        }
    }
    pub fn superclass() -> Option<Arc<Class>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> *mut Class =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_superclass,
            );
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    pub fn conformsToProtocol_(protocol: Option<&Protocol>) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, *mut Protocol) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_conformsToProtocol_,
                protocol
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            _ret
        }
    }
    pub fn description() -> Option<Arc<NSString>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> *mut NSString =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_description,
            );
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    pub fn instancesRespondToSelector_(aSelector: SelectorRef) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, SelectorRef) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_instancesRespondToSelector_,
                aSelector,
            );
            _ret
        }
    }
    pub fn hash() -> usize {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> usize =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_hash,
            );
            _ret
        }
    }
    pub fn debugDescription() -> Option<Arc<NSString>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> *mut NSString =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_debugDescription,
            );
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    pub fn isSubclassOfClass_(aClass: Option<&Class>) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, *mut Class) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_isSubclassOfClass_,
                aClass
                    .as_ref()
                    .map_or(ptr::null_mut(), |r| *r as *const _ as *mut _),
            );
            _ret
        }
    }
    pub fn resolveClassMethod_(sel: SelectorRef) -> bool {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, SelectorRef) -> bool =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_resolveClassMethod_,
                sel,
            );
            _ret
        }
    }
    pub fn instanceMethodForSelector_(aSelector: SelectorRef) -> Option<extern "C" fn() -> ()> {
        unsafe {
            let send: unsafe extern "C" fn(
                *mut Object,
                SelectorRef,
                SelectorRef,
            ) -> Option<extern "C" fn() -> ()> = mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                <Self as ObjCClass>::classref().0 as *const Object as *mut _,
                SEL_instanceMethodForSelector_,
                aSelector,
            );
            _ret
        }
    }
    pub fn copy(&self) -> Option<Arc<Object>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> *mut Object =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(self as *const Self as *mut Self as *mut _, SEL_copy);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    pub fn doesNotRecognizeSelector_(&self, aSelector: SelectorRef) -> () {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, SelectorRef) -> () =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_doesNotRecognizeSelector_,
                aSelector,
            );
            _ret
        }
    }
    pub fn forwardingTargetForSelector_(&self, aSelector: SelectorRef) -> Option<Arc<Object>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef, SelectorRef) -> *mut Object =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_forwardingTargetForSelector_,
                aSelector,
            );
            objc_retainAutoreleasedReturnValue(_ret as *mut _);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    pub fn mutableCopy(&self) -> Option<Arc<Object>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> *mut Object =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(self as *const Self as *mut Self as *mut _, SEL_mutableCopy);
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    pub fn finalize(&self) -> () {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> () =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(self as *const Self as *mut Self as *mut _, SEL_finalize);
            _ret
        }
    }
    pub fn new() -> Option<Arc<Self>> {
        unsafe {
            let send: unsafe extern "C" fn(*mut Object, SelectorRef) -> *mut Self =
                mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                objc_allocWithZone(<Self as ObjCClass>::classref()),
                SEL_init,
            );
            let _ret = Arc::new(_ret);
            _ret
        }
    }
    pub fn methodForSelector_(&self, aSelector: SelectorRef) -> Option<extern "C" fn() -> ()> {
        unsafe {
            let send: unsafe extern "C" fn(
                *mut Object,
                SelectorRef,
                SelectorRef,
            ) -> Option<extern "C" fn() -> ()> = mem::transmute(objc_msgSend as *const u8);
            let _ret = send(
                self as *const Self as *mut Self as *mut _,
                SEL_methodForSelector_,
                aSelector,
            );
            _ret
        }
    }
}
