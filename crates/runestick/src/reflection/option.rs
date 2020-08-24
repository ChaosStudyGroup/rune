//! Trait implementations for Option<T>.

use crate::access::{RawRefGuard, Ref};
use crate::reflection::{FromValue, ReflectValueType, ToValue, UnsafeFromValue};
use crate::value::{Value, ValueType, ValueTypeInfo};
use crate::vm::{Vm, VmError};

impl<T> ReflectValueType for Option<T> {
    type Owned = Option<T>;

    fn value_type() -> ValueType {
        ValueType::Option
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::Option
    }
}

impl<'a, T> ReflectValueType for &'a Option<T> {
    type Owned = Option<T>;

    fn value_type() -> ValueType {
        ValueType::Option
    }

    fn value_type_info() -> ValueTypeInfo {
        ValueTypeInfo::Option
    }
}

impl<T> ToValue for Option<T>
where
    T: ToValue,
{
    fn to_value(self, vm: &mut Vm) -> Result<Value, VmError> {
        Ok(match self {
            Some(some) => {
                let value = some.to_value(vm)?;
                vm.option_allocate(Some(value))
            }
            None => vm.option_allocate(None),
        })
    }
}

impl<T> FromValue for Option<T>
where
    T: FromValue,
{
    fn from_value(value: Value, vm: &mut Vm) -> Result<Self, VmError> {
        match value {
            Value::Option(slot) => {
                let option = vm.option_take(slot)?;

                Ok(match option {
                    Some(some) => Some(T::from_value(some, vm)?),
                    None => None,
                })
            }
            actual => Err(VmError::ExpectedOption {
                actual: actual.type_info(vm)?,
            }),
        }
    }
}

impl<'a> UnsafeFromValue for &'a Option<Value> {
    type Output = *const Option<Value>;
    type Guard = RawRefGuard;

    unsafe fn unsafe_from_value(
        value: Value,
        vm: &mut Vm,
    ) -> Result<(Self::Output, Self::Guard), VmError> {
        let slot = value.into_option(vm)?;
        Ok(Ref::unsafe_into_ref(vm.option_ref(slot)?))
    }

    unsafe fn to_arg(output: Self::Output) -> Self {
        &*output
    }
}
