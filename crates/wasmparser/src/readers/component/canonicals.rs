use crate::limits::MAX_WASM_CANONICAL_OPTIONS;
use crate::{BinaryReader, FromReader, Result, SectionLimited};

/// Represents options for component functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanonicalOption {
    /// The string types in the function signature are UTF-8 encoded.
    UTF8,
    /// The string types in the function signature are UTF-16 encoded.
    UTF16,
    /// The string types in the function signature are compact UTF-16 encoded.
    CompactUTF16,
    /// The memory to use if the lifting or lowering of a function requires memory access.
    ///
    /// The value is an index to a core memory.
    Memory(u32),
    /// The realloc function to use if the lifting or lowering of a function requires memory
    /// allocation.
    ///
    /// The value is an index to a core function of type `(func (param i32 i32 i32 i32) (result i32))`.
    Realloc(u32),
    /// The post-return function to use if the lifting of a function requires
    /// cleanup after the function returns.
    PostReturn(u32),
    /// TODO: docs
    Async,
    /// TODO: docs
    Callback(u32),
}

/// Represents a canonical function in a WebAssembly component.
#[derive(Debug, Clone)]
pub enum CanonicalFunction {
    /// The function lifts a core WebAssembly function to the canonical ABI.
    Lift {
        /// The index of the core WebAssembly function to lift.
        core_func_index: u32,
        /// The index of the lifted function's type.
        type_index: u32,
        /// The canonical options for the function.
        options: Box<[CanonicalOption]>,
    },
    /// The function lowers a canonical ABI function to a core WebAssembly function.
    Lower {
        /// The index of the function to lower.
        func_index: u32,
        /// The canonical options for the function.
        options: Box<[CanonicalOption]>,
    },
    /// A function which creates a new owned handle to a resource.
    ResourceNew {
        /// The type index of the resource that's being created.
        resource: u32,
    },
    /// A function which is used to drop resource handles of the specified type.
    ResourceDrop {
        /// The type index of the resource that's being dropped.
        resource: u32,
    },
    /// A function which returns the underlying i32-based representation of the
    /// specified resource.
    ResourceRep {
        /// The type index of the resource that's being accessed.
        resource: u32,
    },
    /// TODO: docs
    AsyncStart {
        /// TODO: docs
        component_type_index: u32,
    },
    /// TODO: docs
    AsyncReturn {
        /// TODO: docs
        component_type_index: u32,
    },
    /// TODO: docs
    FutureNew {
        /// TODO: docs
        ty: u32,
        /// TODO: docs
        memory: u32,
    },
    /// TODO: docs
    FutureSend {
        /// TODO: docs
        ty: u32,
        /// TODO: docs
        options: Box<[CanonicalOption]>,
    },
    /// TODO: docs
    FutureReceive {
        /// TODO: docs
        ty: u32,
        /// TODO: docs
        options: Box<[CanonicalOption]>,
    },
    /// TODO: docs
    FutureDropSender {
        /// TODO: docs
        ty: u32,
    },
    /// TODO: docs
    FutureDropReceiver {
        /// TODO: docs
        ty: u32,
    },
    /// TODO: docs
    StreamNew {
        /// TODO: docs
        ty: u32,
        /// TODO: docs
        memory: u32,
    },
    /// TODO: docs
    StreamSend {
        /// TODO: docs
        ty: u32,
        /// TODO: docs
        options: Box<[CanonicalOption]>,
    },
    /// TODO: docs
    StreamReceive {
        /// TODO: docs
        ty: u32,
        /// TODO: docs
        options: Box<[CanonicalOption]>,
    },
    /// TODO: docs
    StreamDropSender {
        /// TODO: docs
        ty: u32,
    },
    /// TODO: docs
    StreamDropReceiver {
        /// TODO: docs
        ty: u32,
    },
    /// TODO: docs
    ErrorDrop,
    /// TODO: docs
    TaskWait {
        /// TODO: docs
        memory: u32,
    },
}

/// A reader for the canonical section of a WebAssembly component.
pub type ComponentCanonicalSectionReader<'a> = SectionLimited<'a, CanonicalFunction>;

impl<'a> FromReader<'a> for CanonicalFunction {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<CanonicalFunction> {
        Ok(match reader.read_u8()? {
            0x00 => match reader.read_u8()? {
                0x00 => {
                    let core_func_index = reader.read_var_u32()?;
                    let options = reader
                        .read_iter(MAX_WASM_CANONICAL_OPTIONS, "canonical options")?
                        .collect::<Result<_>>()?;
                    let type_index = reader.read_var_u32()?;
                    CanonicalFunction::Lift {
                        core_func_index,
                        options,
                        type_index,
                    }
                }
                x => return reader.invalid_leading_byte(x, "canonical function lift"),
            },
            0x01 => match reader.read_u8()? {
                0x00 => CanonicalFunction::Lower {
                    func_index: reader.read_var_u32()?,
                    options: reader
                        .read_iter(MAX_WASM_CANONICAL_OPTIONS, "canonical options")?
                        .collect::<Result<_>>()?,
                },
                x => return reader.invalid_leading_byte(x, "canonical function lower"),
            },
            0x02 => CanonicalFunction::ResourceNew {
                resource: reader.read()?,
            },
            0x03 => CanonicalFunction::ResourceDrop {
                resource: reader.read()?,
            },
            0x04 => CanonicalFunction::ResourceRep {
                resource: reader.read()?,
            },
            0x05 => CanonicalFunction::AsyncStart {
                component_type_index: reader.read()?,
            },
            0x06 => CanonicalFunction::AsyncReturn {
                component_type_index: reader.read()?,
            },
            0x07 => CanonicalFunction::FutureNew {
                ty: reader.read()?,
                memory: reader.read()?,
            },
            0x08 => CanonicalFunction::FutureSend {
                ty: reader.read()?,
                options: reader
                    .read_iter(MAX_WASM_CANONICAL_OPTIONS, "canonical options")?
                    .collect::<Result<_>>()?,
            },
            0x09 => CanonicalFunction::FutureReceive {
                ty: reader.read()?,
                options: reader
                    .read_iter(MAX_WASM_CANONICAL_OPTIONS, "canonical options")?
                    .collect::<Result<_>>()?,
            },
            0x0a => CanonicalFunction::FutureDropSender { ty: reader.read()? },
            0x0b => CanonicalFunction::FutureDropReceiver { ty: reader.read()? },
            0x0c => CanonicalFunction::StreamNew {
                ty: reader.read()?,
                memory: reader.read()?,
            },
            0x0d => CanonicalFunction::StreamSend {
                ty: reader.read()?,
                options: reader
                    .read_iter(MAX_WASM_CANONICAL_OPTIONS, "canonical options")?
                    .collect::<Result<_>>()?,
            },
            0x0e => CanonicalFunction::StreamReceive {
                ty: reader.read()?,
                options: reader
                    .read_iter(MAX_WASM_CANONICAL_OPTIONS, "canonical options")?
                    .collect::<Result<_>>()?,
            },
            0x0f => CanonicalFunction::StreamDropSender { ty: reader.read()? },
            0x10 => CanonicalFunction::StreamDropReceiver { ty: reader.read()? },
            0x11 => CanonicalFunction::ErrorDrop,
            0x12 => CanonicalFunction::TaskWait {
                memory: reader.read()?,
            },
            x => return reader.invalid_leading_byte(x, "canonical function"),
        })
    }
}

impl<'a> FromReader<'a> for CanonicalOption {
    fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self> {
        Ok(match reader.read_u8()? {
            0x00 => CanonicalOption::UTF8,
            0x01 => CanonicalOption::UTF16,
            0x02 => CanonicalOption::CompactUTF16,
            0x03 => CanonicalOption::Memory(reader.read_var_u32()?),
            0x04 => CanonicalOption::Realloc(reader.read_var_u32()?),
            0x05 => CanonicalOption::PostReturn(reader.read_var_u32()?),
            0x06 => CanonicalOption::Async,
            0x07 => CanonicalOption::Callback(reader.read_var_u32()?),
            x => return reader.invalid_leading_byte(x, "canonical option"),
        })
    }
}
