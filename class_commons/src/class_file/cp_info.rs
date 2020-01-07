//! Java Virtual Machine instructions do not rely on the run-time layout of classes,
//! interfaces, class instances, or arrays. Instead, instructions refer to symbolic
//! information in the constant_pool table.
//!
//! All constant_pool table entries have the following general format:
//!
//! ```text
//! cp_info {
//!     u1 tag;
//!     u1 info[];
//! }
//! ```
//! Each item in the constant_pool table must begin with a 1-byte tag indicating the
//! kind of cp_info entry. The contents of the info array vary with the value of tag.
//! The valid tags and their values are listed in the following table. Each tag byte must be
//! followed by two or more bytes giving information about the specific constant. The
//! format of the additional information varies with the tag value.
//!
//! |   Constant Type    |  Value  |
//! |--------------------|:-------:|
//! | UTF8               |    1    |
//! | Integer            |    3    |
//! | Float              |    4    |
//! | Long               |    5    |
//! | Double             |    6    |
//! | Class              |    7    |
//! | String             |    8    |
//! | FieldRef           |    9    |
//! | MethodRef          |   10    |
//! | InterfaceMethodRef |   11    |
//! | NameAndType        |   12    |
//! | MethodHandle       |   15    |
//! | MethodType         |   16    |
//! | InvokeDynamic      |   18    |
//! | Module             |   19    |
//! | Package            |   20    |

#[derive(Debug, PartialEq)]
pub enum ConstantInfo {
    /// The CONSTANT_Class_info structure is used to represent a class or an interface:
    ///
    /// ```text
    /// CONSTANT_Class_info {
    ///     u1 tag;
    ///     u2 name_index;
    /// }
    /// ```
    //
    /// The items of the CONSTANT_Class_info structure are as follows:
    ///
    /// # name_index
    ///
    /// The value of the name_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Utf8_info structure (§4.4.7) representing a valid binary class or
    /// interface name encoded in internal form (§4.2.1).
    ///
    /// Because arrays are objects, the opcodes anewarray and multianewarray - but
    /// not the opcode new - can reference array "classes" via CONSTANT_Class_info
    /// structures in the constant_pool table. For such array classes, the name of the class
    /// is the descriptor of the array type (§4.3.2).
    //
    /// For example, the class name representing the two-dimensional array type int[][] is [[I,
    /// while the class name representing the type Thread[] is [Ljava/lang/Thread;.
    ///
    /// An array type descriptor is valid only if it represents 255 or fewer dimensions.
    ClassInfo { name_index: u16 },
    /// The items of this structure are as follows:
    ///
    /// # class_index
    ///
    /// The value of the class_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Class_info structure (§4.4.1) representing a class or interface type
    /// that has the field or method as a member.
    ///
    /// The class_index item of a CONSTANT_Fieldref_info structure may be either
    /// a class type or an interface type.
    ///
    /// # name_and_type_index
    ///
    /// The value of the name_and_type_index item must be a valid index into
    /// the constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_NameAndType_info structure (§4.4.6). This constant_pool entry
    /// indicates the name and descriptor of the field or method.
    ///
    /// In a CONSTANT_Fieldref_info, the indicated descriptor must be a field
    /// descriptor (§4.3.2). Otherwise, the indicated descriptor must be a method
    /// descriptor (§4.3.3).
    FieldRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// The items of this structure are as follows:
    ///
    /// # class_index
    ///
    /// The value of the class_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Class_info structure (§4.4.1) representing a class or interface type
    /// that has the field or method as a member.
    ///
    /// The class_index item of a CONSTANT_Methodref_info structure must be a
    /// class type, not an interface type.
    ///
    /// # name_and_type_index
    ///
    /// The value of the name_and_type_index item must be a valid index into
    /// the constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_NameAndType_info structure (§4.4.6). This constant_pool entry
    /// indicates the name and descriptor of the field or method.
    ///
    /// If the name of the method of a CONSTANT_Methodref_info structure begins
    /// with a '<' ('\u003c'), then the name must be the special name <init>,
    /// representing an instance initialization method (§2.9). The return type of such
    /// a method must be void.
    MethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// The items of this structure are as follows:
    ///
    /// # class_index
    ///
    /// The value of the class_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Class_info structure (§4.4.1) representing a class or interface type
    /// that has the field or method as a member.
    ///
    /// The class_index item of a CONSTANT_InterfaceMethodref_info structure
    /// must be an interface type.
    ///
    /// # name_and_type_index
    ///
    /// The value of the name_and_type_index item must be a valid index into
    /// the constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_NameAndType_info structure (§4.4.6). This constant_pool entry
    /// indicates the name and descriptor of the field or method.
    InterfaceMethodRefInfo {
        class_index: u16,
        name_and_type_index: u16,
    },
    /// The items of the CONSTANT_String_info structure are as follows:
    ///
    /// # string_index
    ///
    /// The value of the string_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Utf8_info structure (§4.4.7) representing the sequence of Unicode
    /// code points to which the String object is to be initialized.
    StringInfo { string_index: u16 },
    /// The items of this structure are as follows:
    ///
    /// # bytes
    ///
    /// The bytes item of the CONSTANT_Integer_info structure represents the value
    /// of the int constant. The bytes of the value are stored in big-endian (high byte
    /// first) order.
    ///
    /// In JustVM we rely on the `i32` data type.
    IntegerInfo { bytes: i32 },
    /// The items of this structure are as follows:
    ///
    /// # bytes
    ///
    /// The bytes item of the CONSTANT_Float_info structure represents the value
    /// of the float constant in IEEE 754 floating-point single format (§2.3.2). The
    /// bytes of the single format representation are stored in big-endian (high byte
    /// first) order.
    ///
    /// The value represented by the CONSTANT_Float_info structure is determined
    /// as follows. The bytes of the value are first converted into an int constant bits.
    ///
    /// Then:
    /// * If bits is 0x7f800000, the float value will be positive infinity.
    /// * If bits is 0xff800000, the float value will be negative infinity.
    /// * If bits is in the range 0x7f800001 through 0x7fffffff or in the range
    /// 0xff800001 through 0xffffffff, the float value will be NaN.
    /// * In all other cases, let s, e, and m be three values that might be computed from
    /// bits:
    ///
    /// ```text
    /// int s = ((bits >> 31) == 0) ? 1 : -1;
    /// int e = ((bits >> 23) & 0xff);
    /// int m = (e == 0) ?
    /// (bits & 0x7fffff) << 1 :
    /// (bits & 0x7fffff) | 0x800000;
    /// ```
    ///
    /// Then the float value equals the result of the mathematical expression `s * m * 2e-150`.
    ///
    /// In JustVM we rely on the `f32` data type.
    FloatInfo { bytes: f32 },
    /// All 8-byte constants take up two entries in the constant_pool table of the class
    /// file. If a CONSTANT_Long_info or CONSTANT_Double_info structure is the item
    /// in the constant_pool table at index n, then the next usable item in the pool is
    /// located at index n+2. The constant_pool index n+1 must be valid but is considered
    /// unusable.
    ///
    /// In retrospect, making 8-byte constants take two constant pool entries was a poor choice.
    ///
    /// # high_bytes, low_bytes
    ///
    /// The unsigned high_bytes and low_bytes items of the CONSTANT_Long_info
    /// structure together represent the value of the long constant
    /// ((long) high_bytes << 32) + low_bytes
    /// where the bytes of each of high_bytes and low_bytes are stored in big-endian
    /// (high byte first) order.
    ///
    /// In JustVM we rely on the `i64` to deal with the behaviour of this data type.
    LongInfo { bytes: i64 },
    /// All 8-byte constants take up two entries in the constant_pool table of the class
    /// file. If a CONSTANT_Long_info or CONSTANT_Double_info structure is the item
    /// in the constant_pool table at index n, then the next usable item in the pool is
    /// located at index n+2. The constant_pool index n+1 must be valid but is considered
    /// unusable.
    ///
    /// In retrospect, making 8-byte constants take two constant pool entries was a poor choice.
    ///
    /// # high_bytes, low_bytes
    ///
    /// The high_bytes and low_bytes items of the CONSTANT_Double_info
    /// structure together represent the double value in IEEE 754 floating-point
    /// double format (§2.3.2). The bytes of each item are stored in big-endian (high
    /// byte first) order.
    ///
    /// The value represented by the CONSTANT_Double_info structure is determined
    /// as follows. The high_bytes and low_bytes items are converted into the long
    /// constant bits, which is equal to
    /// ((long) high_bytes << 32) + low_bytes
    /// Then:
    /// * If bits is 0x7ff0000000000000L, the double value will be positive infinity.
    ///
    /// * If bits is 0xfff0000000000000L, the double value will be negative infinity.
    ///
    /// * If bits is in the range 0x7ff0000000000001L through 0x7fffffffffffffffL
    /// or in the range 0xfff0000000000001L through 0xffffffffffffffffL, the
    /// double value will be NaN.
    ///
    /// * In all other cases, let s, e, and m be three values that might be computed from
    /// bits:
    ///
    /// ```text
    /// int s = ((bits >> 63) == 0) ? 1 : -1;
    /// int e = (int)((bits >> 52) & 0x7ffL);
    /// long m = (e == 0) ?
    /// (bits & 0xfffffffffffffL) << 1 :
    /// (bits & 0xfffffffffffffL) | 0x10000000000000L;
    /// ```
    ///
    /// Then the floating-point value equals the double value of the mathematical
    /// expression s * m * 2e-1075.
    ///
    /// In JustVM we rely on the `f64` to deal with the behaviour of this data type.
    DoubleInfo { bytes: f64 },
    /// The CONSTANT_NameAndType_info structure is used to represent a field or method,
    /// without indicating which class or interface type it belongs to:
    ///
    /// # name_index
    ///
    /// The value of the name_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Utf8_info structure (§4.4.7) representing either the special method
    /// name <init> (§2.9) or a valid unqualified name denoting a field or method
    /// (§4.2.2).
    ///
    /// # descriptor_index
    //
    /// The value of the descriptor_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Utf8_info structure (§4.4.7) representing a valid field descriptor
    /// or method descriptor (§4.3.2, §4.3.3)
    NameAndTypeInfo {
        name_index: u16,
        descriptor_index: u16,
    },
    /// The CONSTANT_Utf8_info structure is used to represent constant string values:
    ///
    /// ```text
    /// CONSTANT_Utf8_info {
    ///     u1 tag;
    ///     u2 length;
    ///     u1 bytes[length];
    /// }
    /// ```
    ///
    /// The items of the CONSTANT_Utf8_info structure are as follows:
    ///
    /// # tag
    ///
    /// The tag item of the CONSTANT_Utf8_info structure has the value
    /// CONSTANT_Utf8 (1).
    ///
    /// # length
    ///
    /// The value of the length item gives the number of bytes in the bytes array (not
    /// the length of the resulting string).
    ///
    /// # bytes[]
    ///
    /// The bytes array contains the bytes of the string.
    ///
    /// No byte may have the value (byte)0.
    ///
    /// No byte may lie in the range (byte)0xf0 to (byte)0xff.
    ///
    /// String content is encoded in modified UTF-8. Modified UTF-8 strings are encoded
    /// so that code point sequences that contain only non-null ASCII characters can be
    /// represented using only 1 byte per code point, but all code points in the Unicode
    /// codespace can be represented. Modified UTF-8 strings are not null-terminated. The
    /// encoding is as follows:
    ///
    /// * Code points in the range '\u0001' to '\u007F' are represented by a single byte:
    ///
    /// ```text
    /// *-----*----------------*
    /// |  0  |   bits 6 - 0   |
    /// *-----*----------------*
    /// ```
    ///
    /// The 7 bits of data in the byte give the value of the code point represented.
    ///
    /// * The null code point ('\u0000') and code points in the range '\u0080' to '\u07FF'
    /// are represented by a pair of bytes x and y :
    ///
    /// ```text
    ///
    ///    *-----*-----*-----*-----------------*
    /// x: |  1  |  1  |  0  |   bits 10 - 6   |
    ///    *-----*-----*-----*-----------------*
    ///
    ///    *-----*-----*----------------------*
    /// y: |  1  |  0  |     bits  5 - 0      |
    ///    *-----*-----*----------------------*
    ///
    /// ```
    ///
    /// The two bytes represent the code point with the value:
    ///
    /// `((x & 0x1f) << 6) + (y & 0x3f)`
    ///
    /// * Code points in the range '\u0800' to '\uFFFF' are represented by 3 bytes x, y,
    /// and z :
    ///
    /// ```text
    ///
    ///    *-----*-----*-----*-----*------------------*
    /// x: |  1  |  1  |  1  |  0  |   bits 15 - 12   |
    ///    *-----*-----*-----*-----*------------------*
    ///
    ///    *-----*-----*------------------------------*
    /// y: |  1  |  0  |        bits  11 - 6          |
    ///    *-----*-----*------------------------------*
    ///
    ///    *-----*-----*------------------------------*
    /// z: |  1  |  0  |         bits  5 - 0          |
    ///    *-----*-----*------------------------------*
    ///
    /// ```
    ///
    /// The three bytes represent the code point with the value:
    ///
    /// `((x & 0xf) << 12) + ((y & 0x3f) << 6) + (z & 0x3f)`
    ///
    /// * Characters with code points above U+FFFF (so-called supplementary
    /// characters) are represented by separately encoding the two surrogate code units
    /// of their UTF-16 representation. Each of the surrogate code units is represented by
    /// three bytes. This means supplementary characters are represented by six bytes,
    /// u, v, w, x, y, and z :
    ///
    /// ```text
    ///
    ///    *-----*-----*-----*-----*-----*-----*-----*------*
    /// u: |  1  |  1  |  1  |  0  |  1  |  1  |  0  |  1   |
    ///    *-----*-----*-----*-----*-----*-----*-----*------*
    ///
    ///    *-----*-----*-----*-----*------------------------*
    /// v: |  1  |  0  |  1  |  0  |   (bits 20 - 16) - 1   |
    ///    *-----*-----*-----*-----*------------------------*
    ///
    ///    *-----*-----*------------------------------------*
    /// w: |  1  |  0  |           bits  15 - 10            |
    ///    *-----*-----*------------------------------------*
    ///
    ///    *-----*-----*-----*-----*-----*-----*-----*------*
    /// x: |  1  |  1  |  1  |  0  |  1  |  1  |  0  |  1   |
    ///    *-----*-----*-----*-----*-----*-----*-----*------*
    ///
    ///    *-----*-----*-----*-----*------------------------*
    /// y: |  1  |  0  |  1  |  1  |     bits  15 - 10      |
    ///    *-----*-----*-----*-----*------------------------*
    ///
    ///    *-----*-----*-----------------------------------*
    /// z: |  1  |  0  |            bits  5 - 0            |
    ///    *-----*-----*-----------------------------------*
    ///
    /// ```
    ///
    /// The six bytes represent the code point with the value:
    ///
    /// `0x10000 + ((v & 0x0f) << 16) + ((w & 0x3f) << 10) + ((y & 0x0f) << 6) + (z & 0x3f)`
    ///
    /// The bytes of multibyte characters are stored in the class file in big-endian (high
    /// byte first) order.
    ///
    /// There are two differences between this format and the "standard" UTF-8 format.
    ///
    /// First, the null character (char)0 is encoded using the 2-byte format rather than the
    /// 1-byte format, so that modified UTF-8 strings never have embedded nulls. Second,
    /// only the 1-byte, 2-byte, and 3-byte formats of standard UTF-8 are used. The Java
    ///
    /// Virtual Machine does not recognize the four-byte format of standard UTF-8; it uses
    /// its own two-times-three-byte format instead.
    ///
    /// For more information regarding the standard UTF-8 format, see Section 3.9 Unicode
    /// Encoding Forms of The Unicode Standard, Version 6.0.0.
    ///
    /// For now, in JustVM we use Rust's `String` type. This might become a problem eventually.
    Utf8Info { data: String },
    /// The CONSTANT_MethodHandle_info structure is used to represent a method handle:
    ///
    /// ```text
    /// CONSTANT_MethodHandle_info {
    ///     u1 tag;
    ///     u1 reference_kind;
    ///     u2 reference_index;
    /// }
    /// ```
    ///
    /// The items of the CONSTANT_MethodHandle_info structure are the following:
    ///
    /// # tag
    ///
    /// The tag item of the CONSTANT_MethodHandle_info structure has the value
    /// CONSTANT_MethodHandle (15).
    ///
    /// # reference_kind
    ///
    /// The value of the reference_kind item must be in the range 1 to 9. The
    /// value denotes the kind of this method handle, which characterizes its bytecode
    /// behavior (§5.4.3.5).
    ///
    /// # reference_index
    ///
    /// The value of the reference_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be as
    /// follows:
    ///
    /// * If the value of the reference_kind item is 1 (REF_getField), 2
    /// (REF_getStatic), 3 (REF_putField), or 4 (REF_putStatic), then the
    /// constant_pool entry at that index must be a CONSTANT_Fieldref_info
    /// (§4.4.2) structure representing a field for which a method handle is to be
    /// created.
    ///
    /// * If the value of the reference_kind item is 5 (REF_invokeVirtual) or 8
    /// (REF_newInvokeSpecial), then the constant_pool entry at that index must
    /// be a CONSTANT_Methodref_info structure (§4.4.2) representing a class's
    /// method or constructor (§2.9) for which a method handle is to be created.
    ///
    /// * If the value of the reference_kind item is 6 (REF_invokeStatic)
    /// or 7 (REF_invokeSpecial), then if the class file version number
    /// is less than 52.0, the constant_pool entry at that index must be
    /// a CONSTANT_Methodref_info structure representing a class's method
    /// for which a method handle is to be created; if the class file
    /// version number is 52.0 or above, the constant_pool entry at that
    /// index must be either a CONSTANT_Methodref_info structure or a
    /// CONSTANT_InterfaceMethodref_info structure (§4.4.2) representing a
    /// class's or interface's method for which a method handle is to be created.
    ///
    /// * If the value of the reference_kind item is 9 (REF_invokeInterface),
    /// then the constant_pool entry at that index must be a
    /// CONSTANT_InterfaceMethodref_info structure representing an interface's
    /// method for which a method handle is to be created.
    ///
    /// If the value of the reference_kind item is 5 (REF_invokeVirtual), 6
    /// (REF_invokeStatic), 7 (REF_invokeSpecial), or 9 (REF_invokeInterface),
    /// the name of the method represented by a CONSTANT_Methodref_info structure
    /// or a CONSTANT_InterfaceMethodref_info structure must not be <init> or
    /// <clinit>.
    ///
    /// If the value is 8 (REF_newInvokeSpecial), the name of the method represented
    /// by a CONSTANT_Methodref_info structure must be <init>.
    MethodHandleInfo {
        reference_kind: MethodHandleReferenceKind,
        reference_index: u16,
    },
    /// The CONSTANT_MethodType_info structure is used to represent a method type:
    ///
    /// ```text
    /// CONSTANT_MethodType_info {
    ///     u1 tag;
    ///     u2 descriptor_index;
    /// }
    /// ```
    ///
    /// The items of the CONSTANT_MethodType_info structure are as follows:
    ///
    /// # tag
    ///
    /// The tag item of the CONSTANT_MethodType_info structure has the value
    /// CONSTANT_MethodType (16).
    ///
    /// # descriptor_index
    ///
    /// The value of the descriptor_index item must be a valid index into the
    /// constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_Utf8_info structure (§4.4.7) representing a method descriptor
    /// (§4.3.3).
    MethodTypeInfo { descriptor_index: u16 },
    /// The CONSTANT_InvokeDynamic_info structure is used by an invokedynamic
    /// instruction (§invokedynamic) to specify a bootstrap method, the dynamic
    /// invocation name, the argument and return types of the call, and optionally, a
    /// sequence of additional constants called static arguments to the bootstrap method.
    ///
    /// ```text
    /// CONSTANT_InvokeDynamic_info {
    ///     u1 tag;
    ///     u2 bootstrap_method_attr_index;
    ///     u2 name_and_type_index;
    /// }
    /// ```
    ///
    /// The items of the CONSTANT_InvokeDynamic_info structure are as follows:
    ///
    /// # tag
    ///
    /// The tag item of the CONSTANT_InvokeDynamic_info structure has the value
    /// CONSTANT_InvokeDynamic (18).
    ///
    /// # bootstrap_method_attr_index
    ///
    /// The value of the bootstrap_method_attr_index item must be a valid index
    /// into the bootstrap_methods array of the bootstrap method table (§4.7.23) of
    /// this class file.
    ///
    /// # name_and_type_index
    ///
    /// The value of the name_and_type_index item must be a valid index into
    /// the constant_pool table. The constant_pool entry at that index must be a
    /// CONSTANT_NameAndType_info structure (§4.4.6) representing a method name
    /// and method descriptor (§4.3.3).
    InvokeDynamicInfo {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    /// The CONSTANT_Module_info structure is used to represent a module:
    ///
    /// ```text
    /// CONSTANT_Module_info {
    ///     u1 tag;
    ///     u2 name_index;
    /// }
    /// ```
    ///
    /// The items of the CONSTANT_Module_info structure are as follows:
    ///
    /// # tag
    ///
    /// The tag item has the value CONSTANT_Module (19).
    ///
    /// # name_index
    ///
    /// The value of the name_index item must be a valid index into
    /// the constant_pool table. The constant_pool entry at that index must
    /// be a CONSTANT_Utf8_info structure (§4.4.7) representing a
    /// valid module name (§4.2.3).
    ///
    /// A CONSTANT_Module_info structure is permitted only in the constant pool
    /// of a class file that declares a module, that is, a ClassFile structure
    /// where the access_flags item has the ACC_MODULE flag set. In all other class
    /// files, a CONSTANT_Module_info structure is illegal.
    ModuleInfo {
        name_index: u16,
    },
    /// The CONSTANT_Package_info structure is used to represent a package exported
    /// or opened by a module:
    ///
    /// ```text
    /// CONSTANT_Package_info {
    ///     u1 tag;
    ///     u2 name_index;
    /// }
    /// ```
    ///
    /// The items of the CONSTANT_Package_info structure are as follows:
    ///
    /// # tag
    ///
    /// The tag item has the value CONSTANT_Package (20).
    ///
    /// # name_index
    ///
    /// The value of the name_index item must be a valid index into the constant_pool
    /// table. The constant_pool entry at that index must be a CONSTANT_Utf8_info
    /// structure (§4.4.7) representing a valid package name encoded in internal form (§4.2.3).
    ///
    /// A CONSTANT_Package_info structure is permitted only in the constant pool of
    /// a class file that declares a module, that is, a ClassFile structure where
    /// the access_flags item has the ACC_MODULE flag set. In all other class files,
    /// a CONSTANT_Package_info structure is illegal.
    PackageInfo {
        name_index: u16,
    },
}

/// This enum represents the possible `MethodHandleInfo` reference_kinds.
/// For more information read `MethodHandleInfo` documentation.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum MethodHandleReferenceKind {
    RefGetField = 1,
    RefGetStatic,
    RefPutField,
    RefPutStatic,
    RefInvokeVirtual,
    RefInvokeStatic,
    RefInvokeSpecial,
    RefNewInvokeSpecial,
    RefInvokeInterface,
}
