use crate::access::{RawRefGuard, Ref};
use crate::reflection::{FromValue, ReflectValueType, ToValue, UnsafeFromValue};
use crate::value::{Object, Value, ValueType, ValueTypeInfo};
use crate::vm::{Vm, VmError};

impl<T> ReflectValueType for Object<T> {
    type Owned = Object<T>;

    fn value_type() -> ValueType {
        ValueType::Object
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::Object
    }
}

impl<'a, T> ReflectValueType for &'a Object<T> {
    type Owned = Object<T>;

    fn value_type() -> ValueType {
        ValueType::Object
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::Object
    }
}

impl<'a, T> ReflectValueType for &'a mut Object<T> {
    type Owned = Object<T>;

    fn value_type() -> ValueType {
        ValueType::Object
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::Object
    }
}

impl<T> FromValue for Object<T>
where
    T: FromValue,
{
    fn from_value(value: Value, vm: &mut Vm) -> Result<Self, VmError> {
        let slot = value.into_object(vm)?;
        let value = vm.object_take(slot)?;
        let mut object = Object::with_capacity(value.len());

        for (key, value) in value {
            object.insert(key, T::from_value(value, vm)?);
        }

        Ok(object)
    }
}

impl<'a> UnsafeFromValue for &'a Object<Value> {
    type Output = *const Object<Value>;
    type Guard = RawRefGuard;

    unsafe fn unsafe_from_value(
        value: Value,
        vm: &mut Vm,
    ) -> Result<(Self::Output, Self::Guard), VmError> {
        let slot = value.into_object(vm)?;
        Ok(Ref::unsafe_into_ref(vm.object_ref(slot)?))
    }

    unsafe fn to_arg(output: Self::Output) -> Self {
        &*output
    }
}

impl<T> ToValue for Object<T>
where
    T: ToValue,
{
    fn to_value(self, vm: &mut Vm) -> Result<Value, VmError> {
        let mut object = Object::with_capacity(self.len());

        for (key, value) in self {
            object.insert(key, value.to_value(vm)?);
        }

        Ok(vm.object_allocate(object))
    }
}
