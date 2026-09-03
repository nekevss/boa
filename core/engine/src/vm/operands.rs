use crate::vm::Instruction;

/// Available Operands types that Boa's VM uses
#[derive(Clone, Debug, PartialEq)]
pub enum Operands {
    None,
    Dst {
        dst: u32,
    },
    LhsRhsDst {
        lhs: u32,
        rhs: u32,
        dst: u32,
    },
    RhsIndexDst {
        rhs: u32,
        index: u32,
        dst: u32,
    },
    SrcDst {
        src: u32,
        dst: u32,
    },
    SetFunctionName {
        function: u32,
        name: u32,
        prefix: u8,
    },
    ValueDst {
        value: f64,
        dst: u32,
    },
    IndexDst {
        index: u32,
        dst: u32,
    },
    Message {
        message: u32,
    },
    Regexp {
        pattern_index: u32,
        flags_index: u32,
        dst: u32,
    },
    Address {
        address: u32,
    },
    AddressValue {
        address: u32,
        value: u32,
    },
    AddressLhsRhs {
        address: u32,
        lhs: u32,
        rhs: u32,
    },
    Case {
        address: u32,
        value: u32,
        condition: u32,
    },
    CallEval {
        argument_count: u32,
        scope_index: u32,
    },
    ScopeIndex {
        scope_index: u32,
    },
    ArgumentCount {
        argument_count: u32,
    },
    BindingIndex {
        binding_index: u32,
    },
    SrcBindingIndex {
        src: u32,
        binding_index: u32,
    },
    DstBindingIndex {
        dst: u32,
        binding_index: u32,
    },
    GetNameGlobal {
        dst: u32,
        binding_index: u32,
        ic_index: u32,
    },
    ObjectValueName {
        object: u32,
        value: u32,
        name_index: u32,
    },
    DstObjectName {
        dst: u32,
        object: u32,
        name_index: u32,
    },
    ObjectProtoValueName {
        object: u32,
        proto: u32,
        value: u32,
        name_index: u32,
    },
    Index {
        index: u32,
    },
    ObjectName {
        object: u32,
        name_index: u32,
    },
    DstValueIc {
        dst: u32,
        value: u32,
        ic_index: u32,
    },
    DstReceiverValueIc {
        dst: u32,
        receiver: u32,
        value: u32,
        ic_index: u32,
    },
    ObjectValueIc {
        object: u32,
        value: u32,
        ic_index: u32,
    },
    ObjectReceiverValueIc {
        object: u32,
        receiver: u32,
        value: u32,
        ic_index: u32,
    },
    DstKeyReceiverObject {
        dst: u32,
        key: u32,
        receiver: u32,
        object: u32,
    },
    ObjectReceiverKeyValue {
        object: u32,
        receiver: u32,
        key: u32,
        value: u32,
    },
    ObjectKeyValue {
        object: u32,
        key: u32,
        value: u32,
    },
    ObjectKey {
        object: u32,
        key: u32,
    },
    ValueDone {
        value: u32,
        done: u32,
    },
    DstClassSuperclass {
        dst: u32,
        class: u32,
        superclass: u32,
    },
    DstPrototypeClass {
        dst: u32,
        prototype: u32,
        class: u32,
    },
    FunctionHome {
        function: u32,
        home: u32,
    },
    Function {
        function: u32,
    },
    ObjectPrototype {
        object: u32,
        prototype: u32,
    },
    Object {
        object: u32,
    },
    ValueArray {
        value: u32,
        array: u32,
    },
    Array {
        array: u32,
    },
    Value {
        value: u32,
    },
    SpecifierOptions {
        specifier: u32,
        options: u32,
    },
    ClassField {
        object: u32,
        name: u32,
        value: u32,
        is_anonymous_function: u32,
    },
    MaybeException {
        has_exception: u32,
        exception: u32,
    },
    Src {
        src: u32,
    },
    IteratorNextReg {
        iterator: u32,
        next: u32,
    },
    Result {
        result: u32,
    },
    ResumeKindValue {
        resume_kind: u32,
        value: u32,
    },
    ValueCalled {
        value: u32,
        called: u32,
    },
    SrcConfigurableName {
        src: u32,
        configurable: u32,
        name_index: u32,
    },
    ConfigurableName {
        configurable: bool,
        name_index: u32,
    },
    ClassNames {
        class: u32,
        name_indices: Box<[u32]>,
    },
    AddressSiteDst {
        address: u32,
        site: u64,
        dst: u32,
    },
    JumpTable {
        index: u32,
        addresses: Box<[u32]>,
    },
    DstValues {
        dst: u32,
        values: Box<[u32]>,
    },
    ObjectSourceExcluded {
        object: u32,
        source: u32,
        excluded_keys: Box<[u32]>,
    },
    SiteDstValues {
        site: u64,
        dst: u32,
        values: Box<[u32]>,
    },
    FunctionObject {
        function_object: u32,
    },
}

impl Operands {
    pub fn from_instruction(instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Pop
            | Instruction::DeleteSuperThrow
            | Instruction::ReThrow
            | Instruction::CheckReturn
            | Instruction::Return
            | Instruction::AsyncGeneratorClose
            | Instruction::CreatePromiseCapability
            | Instruction::PopEnvironment
            | Instruction::IncrementLoopIteration
            | Instruction::IteratorNext
            | Instruction::SuperCallDerived
            | Instruction::CallSpread
            | Instruction::NewSpread
            | Instruction::SuperCallSpread
            | Instruction::PopPrivateEnvironment
            | Instruction::Generator
            | Instruction::AsyncGenerator => Operands::None,

            Instruction::SetRegisterFromAccumulator { dst }
            | Instruction::PopIntoRegister { dst }
            | Instruction::PushZero { dst }
            | Instruction::PushOne { dst }
            | Instruction::PushNan { dst }
            | Instruction::PushPositiveInfinity { dst }
            | Instruction::PushNegativeInfinity { dst }
            | Instruction::PushNull { dst }
            | Instruction::PushTrue { dst }
            | Instruction::PushFalse { dst }
            | Instruction::PushUndefined { dst }
            | Instruction::Exception { dst }
            | Instruction::This { dst }
            | Instruction::NewTarget { dst }
            | Instruction::ImportMeta { dst }
            | Instruction::CreateMappedArgumentsObject { dst }
            | Instruction::CreateUnmappedArgumentsObject { dst }
            | Instruction::RestParameterInit { dst }
            | Instruction::PushNewArray { dst } => Operands::Dst { dst: **dst },

            Instruction::Add { lhs, rhs, dst }
            | Instruction::Sub { lhs, rhs, dst }
            | Instruction::Div { lhs, rhs, dst }
            | Instruction::Mul { lhs, rhs, dst }
            | Instruction::Mod { lhs, rhs, dst }
            | Instruction::Pow { lhs, rhs, dst }
            | Instruction::ShiftRight { lhs, rhs, dst }
            | Instruction::ShiftLeft { lhs, rhs, dst }
            | Instruction::UnsignedShiftRight { lhs, rhs, dst }
            | Instruction::BitOr { lhs, rhs, dst }
            | Instruction::BitAnd { lhs, rhs, dst }
            | Instruction::BitXor { lhs, rhs, dst }
            | Instruction::In { lhs, rhs, dst }
            | Instruction::Eq { lhs, rhs, dst }
            | Instruction::StrictEq { lhs, rhs, dst }
            | Instruction::NotEq { lhs, rhs, dst }
            | Instruction::StrictNotEq { lhs, rhs, dst }
            | Instruction::GreaterThan { lhs, rhs, dst }
            | Instruction::GreaterThanOrEq { lhs, rhs, dst }
            | Instruction::LessThan { lhs, rhs, dst }
            | Instruction::LessThanOrEq { lhs, rhs, dst }
            | Instruction::InstanceOf { lhs, rhs, dst } => Operands::LhsRhsDst {
                lhs: **lhs,
                rhs: **rhs,
                dst: **dst,
            },

            Instruction::InPrivate { dst, index, rhs } => Operands::RhsIndexDst {
                rhs: **rhs,
                index: **index,
                dst: **dst,
            },

            Instruction::Inc { src, dst }
            | Instruction::Dec { src, dst }
            | Instruction::Move { src, dst }
            | Instruction::ToPropertyKey { src, dst } => Operands::SrcDst {
                src: u32::from(*src),
                dst: **dst,
            },

            Instruction::SetFunctionName {
                function,
                name,
                prefix,
            } => Operands::SetFunctionName {
                function: **function,
                name: **name,
                prefix: u32::from(*prefix) as u8,
            },

            Instruction::PushInt8 { value, dst } => Operands::ValueDst {
                value: f64::from(*value),
                dst: **dst,
            },
            Instruction::PushInt16 { value, dst } => Operands::ValueDst {
                value: f64::from(*value),
                dst: **dst,
            },
            Instruction::PushInt32 { value, dst } => Operands::ValueDst {
                value: f64::from(*value),
                dst: **dst,
            },
            Instruction::PushFloat { value, dst } => Operands::ValueDst {
                value: f64::from(*value),
                dst: **dst,
            },
            Instruction::PushDouble { value, dst } => Operands::ValueDst {
                value: *value,
                dst: **dst,
            },

            Instruction::PushLiteral { index, dst }
            | Instruction::ThisForObjectEnvironmentName { index, dst }
            | Instruction::GetFunction { index, dst }
            | Instruction::HasRestrictedGlobalProperty { index, dst }
            | Instruction::CanDeclareGlobalFunction { index, dst }
            | Instruction::CanDeclareGlobalVar { index, dst }
            | Instruction::GetArgument { index, dst } => Operands::IndexDst {
                index: **index,
                dst: **dst,
            },

            Instruction::ThrowNewTypeError { message }
            | Instruction::ThrowNewSyntaxError { message }
            | Instruction::ThrowNewReferenceError { message } => {
                Operands::Message { message: **message }
            }

            Instruction::PushRegexp {
                pattern_index,
                flags_index,
                dst,
            } => Operands::Regexp {
                pattern_index: **pattern_index,
                flags_index: **flags_index,
                dst: **dst,
            },

            Instruction::Jump { address } => Operands::Address {
                address: u32::from(*address),
            },

            Instruction::JumpIfTrue { address, value }
            | Instruction::JumpIfFalse { address, value }
            | Instruction::JumpIfNotUndefined { address, value }
            | Instruction::JumpIfNullOrUndefined { address, value }
            | Instruction::LogicalAnd { address, value }
            | Instruction::LogicalOr { address, value }
            | Instruction::Coalesce { address, value } => Operands::AddressValue {
                address: u32::from(*address),
                value: **value,
            },

            Instruction::JumpIfNotLessThan { address, lhs, rhs }
            | Instruction::JumpIfNotLessThanOrEqual { address, lhs, rhs }
            | Instruction::JumpIfNotGreaterThan { address, lhs, rhs }
            | Instruction::JumpIfNotGreaterThanOrEqual { address, lhs, rhs }
            | Instruction::JumpIfNotEqual { address, lhs, rhs } => Operands::AddressLhsRhs {
                address: u32::from(*address),
                lhs: **lhs,
                rhs: **rhs,
            },

            Instruction::Case {
                address,
                value,
                condition,
            } => Operands::Case {
                address: u32::from(*address),
                value: **value,
                condition: **condition,
            },

            Instruction::CallEval {
                argument_count,
                scope_index,
            } => Operands::CallEval {
                argument_count: **argument_count,
                scope_index: **scope_index,
            },

            Instruction::CallEvalSpread { scope_index }
            | Instruction::PushScope { scope_index } => Operands::ScopeIndex {
                scope_index: **scope_index,
            },

            Instruction::Call { argument_count }
            | Instruction::New { argument_count }
            | Instruction::SuperCall { argument_count } => Operands::ArgumentCount {
                argument_count: **argument_count,
            },

            Instruction::DefVar { binding_index } | Instruction::GetLocator { binding_index } => {
                Operands::BindingIndex {
                    binding_index: **binding_index,
                }
            }

            Instruction::DefInitVar { src, binding_index }
            | Instruction::PutLexicalValue { src, binding_index }
            | Instruction::SetName { src, binding_index } => Operands::SrcBindingIndex {
                src: u32::from(*src),
                binding_index: **binding_index,
            },

            Instruction::GetName { dst, binding_index }
            | Instruction::GetNameAndLocator { dst, binding_index }
            | Instruction::GetNameOrUndefined { dst, binding_index }
            | Instruction::DeleteName { dst, binding_index } => Operands::DstBindingIndex {
                dst: **dst,
                binding_index: **binding_index,
            },

            Instruction::GetNameGlobal {
                dst,
                binding_index,
                ic_index,
            } => Operands::GetNameGlobal {
                dst: **dst,
                binding_index: **binding_index,
                ic_index: **ic_index,
            },

            Instruction::DefineOwnPropertyByName {
                object,
                value,
                name_index,
            }
            | Instruction::SetPropertyGetterByName {
                object,
                value,
                name_index,
            }
            | Instruction::SetPropertySetterByName {
                object,
                value,
                name_index,
            }
            | Instruction::DefinePrivateField {
                object,
                value,
                name_index,
            }
            | Instruction::SetPrivateMethod {
                object,
                value,
                name_index,
            }
            | Instruction::SetPrivateSetter {
                object,
                value,
                name_index,
            }
            | Instruction::SetPrivateGetter {
                object,
                value,
                name_index,
            }
            | Instruction::PushClassPrivateGetter {
                object,
                value,
                name_index,
            }
            | Instruction::PushClassPrivateSetter {
                object,
                value,
                name_index,
            }
            | Instruction::DefineClassStaticMethodByName {
                object,
                value,
                name_index,
            }
            | Instruction::DefineClassMethodByName {
                object,
                value,
                name_index,
            }
            | Instruction::DefineClassStaticGetterByName {
                object,
                value,
                name_index,
            }
            | Instruction::DefineClassGetterByName {
                object,
                value,
                name_index,
            }
            | Instruction::DefineClassStaticSetterByName {
                object,
                value,
                name_index,
            }
            | Instruction::DefineClassSetterByName {
                object,
                value,
                name_index,
            }
            | Instruction::SetPrivateField {
                object,
                value,
                name_index,
            }
            | Instruction::PushClassFieldPrivate {
                object,
                value,
                name_index,
            } => Operands::ObjectValueName {
                object: **object,
                value: **value,
                name_index: **name_index,
            },
            Instruction::GetPrivateField {
                dst,
                object,
                name_index,
            } => Operands::DstObjectName {
                dst: **dst,
                object: **object,
                name_index: **name_index,
            },
            Instruction::PushClassPrivateMethod {
                object,
                proto,
                value,
                name_index,
            } => Operands::ObjectProtoValueName {
                object: **object,
                proto: **proto,
                value: **value,
                name_index: **name_index,
            },
            Instruction::ThrowMutateImmutable { index } => Operands::Index { index: **index },
            Instruction::DeletePropertyByName { object, name_index }
            | Instruction::GetMethod { object, name_index } => Operands::ObjectName {
                object: **object,
                name_index: **name_index,
            },
            Instruction::GetLengthProperty {
                dst,
                value,
                ic_index,
            }
            | Instruction::GetPropertyByName {
                dst,
                value,
                ic_index,
            } => Operands::DstValueIc {
                dst: **dst,
                value: **value,
                ic_index: **ic_index,
            },
            Instruction::GetPropertyByNameWithThis {
                dst,
                receiver,
                value,
                ic_index,
            } => Operands::DstReceiverValueIc {
                dst: **dst,
                receiver: **receiver,
                value: **value,
                ic_index: **ic_index,
            },
            Instruction::SetPropertyByName {
                value,
                object,
                ic_index,
            } => Operands::ObjectValueIc {
                object: **object,
                value: **value,
                ic_index: **ic_index,
            },
            Instruction::SetPropertyByNameWithThis {
                value,
                receiver,
                object,
                ic_index,
            } => Operands::ObjectReceiverValueIc {
                object: **object,
                receiver: **receiver,
                value: **value,
                ic_index: **ic_index,
            },
            Instruction::GetPropertyByValue {
                dst,
                key,
                receiver,
                object,
            }
            | Instruction::GetPropertyByValuePush {
                dst,
                key,
                receiver,
                object,
            } => Operands::DstKeyReceiverObject {
                dst: **dst,
                key: **key,
                receiver: **receiver,
                object: **object,
            },
            Instruction::SetPropertyByValue {
                value,
                key,
                receiver,
                object,
            } => Operands::ObjectReceiverKeyValue {
                object: **object,
                receiver: **receiver,
                key: **key,
                value: **value,
            },
            Instruction::DefineOwnPropertyByValue { value, key, object }
            | Instruction::DefineClassStaticMethodByValue { value, key, object }
            | Instruction::DefineClassMethodByValue { value, key, object }
            | Instruction::SetPropertyGetterByValue { value, key, object }
            | Instruction::DefineClassStaticGetterByValue { value, key, object }
            | Instruction::DefineClassGetterByValue { value, key, object }
            | Instruction::SetPropertySetterByValue { value, key, object }
            | Instruction::DefineClassStaticSetterByValue { value, key, object }
            | Instruction::DefineClassSetterByValue { value, key, object } => {
                Operands::ObjectKeyValue {
                    object: **object,
                    key: **key,
                    value: **value,
                }
            }
            Instruction::DeletePropertyByValue { key, object } => Operands::ObjectKey {
                object: **object,
                key: **key,
            },
            Instruction::CreateIteratorResult { value, done } => Operands::ValueDone {
                value: **value,
                done: **done,
            },
            Instruction::PushClassPrototype {
                dst,
                class,
                superclass,
            } => Operands::DstClassSuperclass {
                dst: **dst,
                class: **class,
                superclass: **superclass,
            },
            Instruction::SetClassPrototype {
                dst,
                prototype,
                class,
            } => Operands::DstPrototypeClass {
                dst: u32::from(**dst),
                prototype: **prototype,
                class: **class,
            },
            Instruction::SetHomeObject { function, home } => Operands::FunctionHome {
                function: **function,
                home: **home,
            },
            Instruction::GetHomeObject { function } => Operands::Function {
                function: **function,
            },
            Instruction::SetPrototype { object, prototype } => Operands::ObjectPrototype {
                object: **object,
                prototype: **prototype,
            },
            Instruction::GetPrototype { object } => Operands::Object { object: **object },
            Instruction::PushValueToArray { value, array } => Operands::ValueArray {
                value: **value,
                array: **array,
            },
            Instruction::PushElisionToArray { array }
            | Instruction::PushIteratorToArray { array } => Operands::Array { array: **array },
            Instruction::TypeOf { value }
            | Instruction::LogicalNot { value }
            | Instruction::Pos { value }
            | Instruction::Neg { value }
            | Instruction::IsObject { value }
            | Instruction::BindThisValue { value }
            | Instruction::BitNot { value } => Operands::Value { value: **value },
            Instruction::ImportCall { specifier, options } => Operands::SpecifierOptions {
                specifier: **specifier,
                options: **options,
            },
            Instruction::PushClassField {
                object,
                name,
                value,
                is_anonymous_function,
            } => Operands::ClassField {
                object: **object,
                name: **name,
                value: **value,
                is_anonymous_function: **is_anonymous_function,
            },
            Instruction::MaybeException {
                has_exception,
                exception,
            } => Operands::MaybeException {
                has_exception: **has_exception,
                exception: **exception,
            },
            Instruction::SetAccumulator { src }
            | Instruction::PushFromRegister { src }
            | Instruction::Throw { src }
            | Instruction::SetNameByLocator { src }
            | Instruction::PushObjectEnvironment { src }
            | Instruction::CreateForInIterator { src }
            | Instruction::GetIterator { src }
            | Instruction::GetAsyncIterator { src }
            | Instruction::ValueNotNullOrUndefined { src }
            | Instruction::GeneratorYield { src }
            | Instruction::AsyncGeneratorYield { src }
            | Instruction::Await { src } => Operands::Src {
                src: u32::from(*src),
            },
            Instruction::IteratorPush { iterator, next }
            | Instruction::IteratorPop { iterator, next } => Operands::IteratorNextReg {
                iterator: **iterator,
                next: **next,
            },
            Instruction::IteratorUpdateResult { result } => Operands::Result { result: **result },
            Instruction::IteratorDone { dst }
            | Instruction::IteratorValue { dst }
            | Instruction::IteratorResult { dst }
            | Instruction::IteratorToArray { dst }
            | Instruction::IteratorStackEmpty { dst }
            | Instruction::PushEmptyObject { dst } => Operands::Dst { dst: **dst },
            Instruction::IteratorFinishAsyncNext { resume_kind, value } => {
                Operands::ResumeKindValue {
                    resume_kind: **resume_kind,
                    value: **value,
                }
            }
            Instruction::IteratorReturn { value, called } => Operands::ValueCalled {
                value: **value,
                called: **called,
            },
            Instruction::CreateGlobalFunctionBinding {
                src,
                configurable,
                name_index,
            } => Operands::SrcConfigurableName {
                src: **src,
                configurable: **configurable,
                name_index: **name_index,
            },
            Instruction::CreateGlobalVarBinding {
                configurable,
                name_index,
            } => Operands::ConfigurableName {
                configurable: u32::from(*configurable) == 1,
                name_index: **name_index,
            },
            Instruction::PushPrivateEnvironment {
                class,
                name_indices,
            } => Operands::ClassNames {
                class: **class,
                name_indices: name_indices
                    .iter()
                    .copied()
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
            Instruction::TemplateLookup { address, site, dst } => Operands::AddressSiteDst {
                address: u32::from(*address),
                site: *site,
                dst: **dst,
            },
            Instruction::JumpTable { index, addresses } => Operands::JumpTable {
                index: *index,
                addresses: addresses
                    .iter()
                    .copied()
                    .map(u32::from)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
            Instruction::ConcatToString { dst, values } => Operands::DstValues {
                dst: **dst,
                values: values
                    .iter()
                    .map(std::ops::Deref::deref)
                    .copied()
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
            Instruction::CopyDataProperties {
                object,
                source,
                excluded_keys,
            } => Operands::ObjectSourceExcluded {
                object: **object,
                source: **source,
                excluded_keys: excluded_keys
                    .iter()
                    .map(std::ops::Deref::deref)
                    .copied()
                    .collect::<Vec<u32>>()
                    .into_boxed_slice(),
            },
            Instruction::TemplateCreate { site, dst, values } => Operands::SiteDstValues {
                site: *site,
                dst: **dst,
                values: values
                    .iter()
                    .copied()
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            },
            Instruction::GetFunctionObject { function_object } => Operands::FunctionObject {
                function_object: **function_object,
            },
            Instruction::Reserved1
            | Instruction::Reserved2
            | Instruction::Reserved3
            | Instruction::Reserved4
            | Instruction::Reserved5
            | Instruction::Reserved6
            | Instruction::Reserved7
            | Instruction::Reserved8
            | Instruction::Reserved9
            | Instruction::Reserved10
            | Instruction::Reserved11
            | Instruction::Reserved12
            | Instruction::Reserved13
            | Instruction::Reserved14
            | Instruction::Reserved15
            | Instruction::Reserved16
            | Instruction::Reserved17
            | Instruction::Reserved18
            | Instruction::Reserved19
            | Instruction::Reserved20
            | Instruction::Reserved21
            | Instruction::Reserved22
            | Instruction::Reserved23
            | Instruction::Reserved24
            | Instruction::Reserved25
            | Instruction::Reserved26
            | Instruction::Reserved27
            | Instruction::Reserved28
            | Instruction::Reserved29
            | Instruction::Reserved30
            | Instruction::Reserved31
            | Instruction::Reserved32
            | Instruction::Reserved33
            | Instruction::Reserved34
            | Instruction::Reserved35
            | Instruction::Reserved36
            | Instruction::Reserved37
            | Instruction::Reserved38
            | Instruction::Reserved39
            | Instruction::Reserved40
            | Instruction::Reserved41
            | Instruction::Reserved42
            | Instruction::Reserved43
            | Instruction::Reserved44
            | Instruction::Reserved45
            | Instruction::Reserved46
            | Instruction::Reserved47
            | Instruction::Reserved48
            | Instruction::Reserved49
            | Instruction::Reserved50
            | Instruction::Reserved51
            | Instruction::Reserved52
            | Instruction::Reserved53
            | Instruction::Reserved54 => unreachable!("Reserved opcodes are unreachable"),
        }
    }
}

impl std::fmt::Display for Operands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => Ok(()),
            Self::Dst { dst } => write!(f, "dst:{dst}"),
            Self::LhsRhsDst { lhs, rhs, dst } => write!(f, "lhs:{lhs}, rhs:{rhs}, dst:{dst}"),
            Self::RhsIndexDst { rhs, index, dst } => {
                write!(f, "rhs:{rhs}, index:{index}, dst:{dst}")
            }
            Self::SrcDst { src, dst } => write!(f, "src:{src}, dst:{dst}"),
            Self::SetFunctionName {
                function,
                name,
                prefix,
            } => {
                let prefix_str = match prefix {
                    1 => "prefix: get",
                    2 => "prefix: set",
                    _ => "prefix:",
                };
                write!(f, "function:{function}, name:{name}, {prefix_str}")
            }
            Self::ValueDst { value, dst } => write!(f, "value:{value}, dst:{dst}"),
            Self::IndexDst { index, dst } => write!(f, "index:{index}, dst:{dst}"),
            Self::Message { message } => write!(f, "message:{message}"),
            Self::Regexp {
                pattern_index,
                flags_index,
                dst,
            } => {
                write!(f, "pattern:{pattern_index}, flags:{flags_index}, dst:{dst}")
            }
            Self::Address { address } => write!(f, "address:{address}"),
            Self::AddressValue { address, value } => write!(f, "value:{value}, address:{address}"),
            Self::AddressLhsRhs { address, lhs, rhs } => {
                write!(f, "lhs:{lhs}, rhs:{rhs}, address:{address}")
            }
            Self::Case {
                address,
                value,
                condition,
            } => {
                write!(f, "value:{value}, condition:{condition}, address:{address}")
            }
            Self::CallEval {
                argument_count,
                scope_index,
            } => {
                write!(
                    f,
                    "argument_count:{argument_count}, scope_index:{scope_index}"
                )
            }
            Self::ScopeIndex { scope_index } => write!(f, "scope_index:{scope_index}"),
            Self::ArgumentCount { argument_count } => write!(f, "argument_count:{argument_count}"),
            Self::BindingIndex { binding_index } => write!(f, "binding_index:{binding_index}"),
            Self::SrcBindingIndex { src, binding_index } => {
                write!(f, "src:{src}, binding_index:{binding_index}")
            }
            Self::DstBindingIndex { dst, binding_index } => {
                write!(f, "dst:{dst}, binding_index:{binding_index}")
            }
            Self::GetNameGlobal {
                dst,
                binding_index,
                ic_index,
            } => {
                write!(
                    f,
                    "dst:{dst}, binding_index:{binding_index}, ic_index:{ic_index}"
                )
            }
            Self::ObjectValueName {
                object,
                value,
                name_index,
            } => {
                write!(f, "object:{object}, value:{value}, name_index:{name_index}")
            }
            Self::DstObjectName {
                dst,
                object,
                name_index,
            } => {
                write!(f, "dst:{dst}, object:{object}, name_index:{name_index}")
            }
            Self::ObjectProtoValueName {
                object,
                proto,
                value,
                name_index,
            } => {
                write!(
                    f,
                    "object:{object}, proto:{proto}, value:{value}, name_index:{name_index}"
                )
            }
            Self::Index { index } => write!(f, "index:{index}"),
            Self::ObjectName { object, name_index } => {
                write!(f, "object:{object}, name_index:{name_index}")
            }
            Self::DstValueIc {
                dst,
                value,
                ic_index,
            } => write!(f, "dst:{dst}, value:{value}, ic:{ic_index}"),
            Self::DstReceiverValueIc {
                dst,
                receiver,
                value,
                ic_index,
            } => {
                write!(
                    f,
                    "dst:{dst}, receiver:{receiver}, value:{value}, ic:{ic_index}"
                )
            }
            Self::ObjectValueIc {
                object,
                value,
                ic_index,
            } => write!(f, "object:{object}, value:{value}, ic:{ic_index}"),
            Self::ObjectReceiverValueIc {
                object,
                receiver,
                value,
                ic_index,
            } => {
                write!(
                    f,
                    "object:{object}, receiver:{receiver}, value:{value}, ic:{ic_index}"
                )
            }
            Self::DstKeyReceiverObject {
                dst,
                key,
                receiver,
                object,
            } => {
                write!(
                    f,
                    "dst:{dst}, object:{object}, receiver:{receiver}, key:{key}"
                )
            }
            Self::ObjectReceiverKeyValue {
                object,
                receiver,
                key,
                value,
            } => {
                write!(
                    f,
                    "object:{object}, receiver:{receiver}, key:{key}, value:{value}"
                )
            }
            Self::ObjectKeyValue { object, key, value } => {
                write!(f, "object:{object}, key:{key}, value:{value}")
            }
            Self::ObjectKey { object, key } => write!(f, "object:{object}, key:{key}"),
            Self::ValueDone { value, done } => write!(f, "value:{value}, done:{done}"),
            Self::DstClassSuperclass {
                dst,
                class,
                superclass,
            } => {
                write!(f, "dst:{dst}, class:{class}, superclass:{superclass}")
            }
            Self::DstPrototypeClass {
                dst,
                prototype,
                class,
            } => {
                write!(f, "dst:{dst}, prototype:{prototype}, class:{class}")
            }
            Self::FunctionHome { function, home } => write!(f, "function:{function}, home:{home}"),
            Self::Function { function } => write!(f, "function:{function}"),
            Self::ObjectPrototype { object, prototype } => {
                write!(f, "object:{object}, prototype:{prototype}")
            }
            Self::Object { object } => write!(f, "object:{object}"),
            Self::ValueArray { value, array } => write!(f, "value:{value}, array:{array}"),
            Self::Array { array } => write!(f, "array:{array}"),
            Self::Value { value } => write!(f, "value:{value}"),
            Self::SpecifierOptions { specifier, options } => {
                write!(f, "specifier:{specifier}, options:{options}")
            }
            Self::ClassField {
                object,
                name,
                value,
                is_anonymous_function,
            } => {
                write!(
                    f,
                    "object:{object}, value:{value}, name:{name}, is_anonymous_function:{is_anonymous_function}"
                )
            }
            Self::MaybeException {
                has_exception,
                exception,
            } => {
                write!(f, "has_exception:{has_exception}, exception:{exception}")
            }
            Self::Src { src } => write!(f, "src:{src}"),
            Self::IteratorNextReg { iterator, next } => {
                write!(f, "iterator:{iterator}, next:{next}")
            }
            Self::Result { result } => write!(f, "result:{result}"),
            Self::ResumeKindValue { resume_kind, value } => {
                write!(f, "resume_kind:{resume_kind}, value:{value}")
            }
            Self::ValueCalled { value, called } => write!(f, "value:{value}, called:{called}"),
            Self::SrcConfigurableName {
                src,
                configurable,
                name_index,
            } => {
                write!(
                    f,
                    "src:{src}, configurable:{configurable}, name_index:{name_index}"
                )
            }
            Self::ConfigurableName {
                configurable,
                name_index,
            } => {
                write!(f, "configurable:{configurable}, name_index:{name_index}")
            }
            Self::ClassNames {
                class,
                name_indices,
            } => write!(f, "class:{class}, names:{name_indices:?}"),
            Self::AddressSiteDst { address, site, dst } => {
                write!(f, "address:{address}, site:{site}, dst:{dst}")
            }
            Self::JumpTable { index, addresses } => {
                use itertools::Itertools;
                write!(
                    f,
                    "index:{index}, jump_table:({})",
                    addresses.iter().format(", ")
                )
            }
            Self::DstValues { dst, values } => write!(f, "dst:{dst}, values:{values:?}"),
            Self::ObjectSourceExcluded {
                object,
                source,
                excluded_keys,
            } => {
                write!(
                    f,
                    "object:{object}, source:{source}, excluded_keys:{excluded_keys:?}"
                )
            }
            Self::SiteDstValues { site, dst, values } => {
                write!(f, "site:{site}, dst:{dst}, values:{values:?}")
            }
            Self::FunctionObject { function_object } => {
                write!(f, "function_object:{function_object}")
            }
        }
    }
}
