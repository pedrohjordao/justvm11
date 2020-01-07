pub mod cp_info;
pub mod field_info;
pub mod method_info;
pub mod attribute_info;

use bitflags::bitflags;

struct CpInfo;

struct FieldInfo;

struct MethodInfo;

struct AttributeInfo;

bitflags! {
    pub struct ClassAccessFlags: u16 {
        const ACC_PUBLIC = 0x0001;
        const ACC_FINAL = 0x0010;
        const ACC_SUPER = 0x0020;
        const ACC_INTERFACE = 0x0200;
        const ACC_ABSTRACT = 0x4000;
        const ACC_SYNTHETIC = 0x1000;
        const ACC_ANNOTATION = 0x2000;
        const ACC_ENUM = 0x4000;
        const ACC_MODULE = 0x8000;
    }
}

pub struct ClassFile {
    /// The magic item supplies the magic number identifying the class file format;
    /// it has the value 0xCAFEBABE.
    magic: u32,
    /// The values of the minor_version and major_version items are the minor and
    /// major version numbers of this class file. Together, a major and a minor version
    /// number determine the version of the class file format. If a class file has major
    /// version number M and minor version number m, we denote the version of its
    /// class file format as M.m. Thus, class file format versions may be ordered
    /// lexicographically, for example, 1.5 < 2.0 < 2.1.
    /// A Java Virtual Machine implementation can support a class file format of
    /// version v if and only if v lies in some contiguous range Mi.0 ≤ v ≤ Mj.m.
    /// The release level of the Java SE platform to which a Java Virtual Machine
    /// implementation conforms is responsible for determining the range.
    // TODO: Split into different structure that handles version info
    minor_version: u16,
    major_version: u16,
    ///  The constant_pool is a table of structures (§4.4) representing various string
    /// constants, class and interface names, field names, and other constants that are
    /// referred to within the ClassFile structure and its substructures. The format of
    /// each constant_pool table entry is indicated by its first "tag" byte.
    /// The constant_pool table is indexed from 1 to constant_pool_count - 1.
    // no count for the constant pool necessary
    cp_info: Vec<CpInfo>,
    /// The value of the access_flags item is a mask of flags used to denote access
    /// permissions to and properties of this class or interface. The interpretation of
    /// each flag, when set, is specified in the table:
    ///
    /// |    Flag Name   | Value  |                                     Interpretation                                |
    /// |----------------|--------|-----------------------------------------------------------------------------------|
    /// | ACC_PUBLIC     | 0x0001 | Declared public; may be accessed from outside its package.                        |
    /// | ACC_FINAL      | 0x0010 | Declared final;  No subclass allowed                                              |
    /// | ACC_SUPER      | 0x0020 | Treat superclass methods specially when invoked by the invokespecial instruction. |
    /// | ACC_INTERFACE  | 0x0200 | Is an Interface; Not a class                                                      |
    /// | ACC_ABSTRACT   | 0x4000 | Declared Abstract; Must not be initialized                                        |
    /// | ACC_SYNTHETIC  | 0x1000 | Declared Synthetic; Not present in the source code                                |
    /// | ACC_ANNOTATION | 0x2000 | Declared as an Annotation type                                                    |
    /// | ACC_ENUM       | 0x4000 | Declared as an Enum type                                                          |
    ///
    /// An interface is distinguished by the ACC_INTERFACE flag being set. If the
    /// ACC_INTERFACE flag is not set, this class file defines a class, not an interface.
    ///
    /// If the ACC_INTERFACE flag is set, the ACC_ABSTRACT flag must also be set, and
    /// the ACC_FINAL, ACC_SUPER, and ACC_ENUM flags set must not be set.
    ///
    /// If the ACC_INTERFACE flag is not set, any of the other flags in Table 4.1-A may
    /// be set except ACC_ANNOTATION. However, such a class file must not have both
    /// its ACC_FINAL and ACC_ABSTRACT flags set (JLS §8.1.1.2).
    ///
    /// The ACC_SUPER flag indicates which of two alternative semantics is to be
    /// expressed by the invokespecial instruction (§invokespecial) if it appears in
    /// this class or interface. Compilers to the instruction set of the Java Virtual
    /// Machine should set the ACC_SUPER flag. In Java SE 8 and above, the Java
    /// Virtual Machine considers the ACC_SUPER flag to be set in every class file,
    /// regardless of the actual value of the flag in the class file and the version of
    /// the class file.
    ///
    /// The ACC_SUPER flag exists for backward compatibility with code compiled by older
    /// compilers for the Java programming language. In JDK releases prior to 1.0.2, the compiler
    /// generated access_flags in which the flag now representing ACC_SUPER had no assigned
    /// meaning, and Oracle's Java Virtual Machine implementation ignored the flag if it was set.
    ///
    /// The ACC_SYNTHETIC flag indicates that this class or interface was generated by
    /// a compiler and does not appear in source code.
    /// An annotation type must have its ACC_ANNOTATION flag set. If the
    ///
    /// ACC_ANNOTATION flag is set, the ACC_INTERFACE flag must also be set.
    ///
    /// The ACC_ENUM flag indicates that this class or its superclass is declared as an
    /// enumerated type.
    ///
    /// All bits of the access_flags item not assigned in the table are reserved for
    /// future use. They should be set to zero in generated class files and should be
    /// ignored by Java Virtual Machine implementations
    access_flags: ClassAccessFlags,
    /// The value of the this_class item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Class_info structure (§4.4.1) representing the class or interface
    /// defined by this class file.
    this_class: u16,
    /// For a class, the value of the super_class item either must be zero or
    /// must be a valid index into the constant_pool table. If the value of the
    /// super_class item is nonzero, the constant_pool entry at that index must
    /// be a CONSTANT_Class_info structure representing the direct superclass of the
    /// class defined by this class file. Neither the direct superclass nor any of its
    /// superclasses may have the ACC_FINAL flag set in the access_flags item of its
    /// ClassFile structure.
    ///
    /// If the value of the super_class item is zero, then this class file must represent
    /// the class Object, the only class or interface without a direct superclass.
    ///
    /// For an interface, the value of the super_class item must always be a valid
    /// index into the constant_pool table. The constant_pool entry at that index
    /// must be a CONSTANT_Class_info structure representing the class Object.
    super_class: u16,
    /// Each value in the interfaces array must be a valid index into
    /// the constant_pool table. The constant_pool entry at each value
    /// of interfaces[i], where 0 ≤ i < interfaces_count, must be a
    /// CONSTANT_Class_info structure representing an interface that is a direct
    /// superinterface of this class or interface type, in the left-to-right order given in
    /// the source for the type.
    // no need for a count again
    interfaces: Vec<u16>,
    /// Each value in the fields table must be a field_info structure (§4.5) giving
    ///  a complete description of a field in this class or interface. The fields table
    /// includes only those fields that are declared by this class or interface. It does
    /// not include items representing fields that are inherited from superclasses or
    /// superinterfaces
    fields: Vec<FieldInfo>,
    /// Each value in the methods table must be a method_info structure (§4.6) giving
    /// a complete description of a method in this class or interface. If neither of the
    /// ACC_NATIVE and ACC_ABSTRACT flags are set in the access_flags item of a
    /// method_info structure, the Java Virtual Machine instructions implementing
    /// the method are also supplied.
    ///
    /// The method_info structures represent all methods declared by this class
    /// or interface type, including instance methods, class methods, instance
    /// initialization methods (§2.9), and any class or interface initialization method
    /// (§2.9). The methods table does not include items representing methods that are
    /// inherited from superclasses or superinterfaces.
    methods: Vec<MethodInfo>,
    /// Each value of the attributes table must be an attribute_info structure
    /// (§4.7).
    ///
    /// The attributes defined by this specification as appearing in the attributes
    /// table of a ClassFile structure are listed in Table 4.7-C.
    ///
    /// The rules concerning attributes defined to appear in the attributes table of a
    /// ClassFile structure are given in §4.7.
    ///
    /// The rules concerning non-predefined attributes in the attributes table of a
    /// ClassFile structure are given in §4.7.1
    attributes: Vec<AttributeInfo>,
}