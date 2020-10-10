use crate::error::{Error, Result};
use crate::serial::{Decodable, Encodable, ReadExt, VarInt};
use crate::vm::{
    AllocType, ConstraintInstruction, CryptoOperation, VariableIndex, VariableRef, ZKVMCircuit,
    ZKVirtualMachine,
};
use crate::{impl_vec, ZKSupervisor};
use std::collections::HashMap;
use std::io;

impl Encodable for ZKSupervisor {
    fn encode<S: io::Write>(&self, mut s: S) -> Result<usize> {
        unimplemented!();
        Ok(0)
    }
}

impl Decodable for ZKSupervisor {
    fn decode<D: io::Read>(mut d: D) -> Result<Self> {
        Ok(Self {
            name: Decodable::decode(&mut d)?,
            vm: ZKVirtualMachine {
                constants: Decodable::decode(&mut d)?,
                alloc: Decodable::decode(&mut d)?,
                ops: Decodable::decode(&mut d)?,
                constraints: Decodable::decode(&mut d)?,

                aux: Vec::new(),
                params: None,
                verifying_key: None,
            },
            params_map: HashMap::new(),
            params: HashMap::new(),
            public_map: HashMap::new(),
        })
    }
}

impl Encodable for (AllocType, VariableIndex) {
    fn encode<S: io::Write>(&self, mut s: S) -> Result<usize> {
        //let len = self.x.encode(&mut s)?;
        //Ok(len + self.y.encode(s)?)
        unimplemented!();
        Ok(0)
    }
}

impl Decodable for (AllocType, VariableIndex) {
    fn decode<D: io::Read>(mut d: D) -> Result<Self> {
        let type_val = ReadExt::read_u8(&mut d)?;
        assert!(type_val == 0 || type_val == 1);
        let alloc_type = if type_val == 0 {
            AllocType::Private
        } else {
            AllocType::Public
        };
        Ok((alloc_type, ReadExt::read_u32(&mut d)? as usize))
    }
}

impl_vec!((AllocType, VariableIndex));

impl Decodable for VariableIndex {
    fn decode<D: io::Read>(mut d: D) -> Result<Self> {
        Ok(ReadExt::read_u32(&mut d)? as Self)
    }
}

impl Encodable for VariableRef {
    fn encode<S: io::Write>(&self, mut s: S) -> Result<usize> {
        unimplemented!();
        Ok(0)
    }
}

impl Decodable for VariableRef {
    fn decode<D: io::Read>(mut d: D) -> Result<Self> {
        let arg_type = ReadExt::read_u8(&mut d)?;
        match arg_type {
            0 => Ok(Self::Aux(Decodable::decode(&mut d)?)),
            1 => Ok(Self::Local(Decodable::decode(&mut d)?)),
            _ => Err(Error::BadVariableRefType),
        }
    }
}

impl Encodable for CryptoOperation {
    fn encode<S: io::Write>(&self, mut s: S) -> Result<usize> {
        unimplemented!();
        Ok(0)
    }
}

impl Decodable for CryptoOperation {
    fn decode<D: io::Read>(mut d: D) -> Result<Self> {
        let op_type = ReadExt::read_u8(&mut d)?;
        match op_type {
            0 => Ok(Self::Set(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            1 => Ok(Self::Mul(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            2 => Ok(Self::Add(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            3 => Ok(Self::Sub(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            4 => Ok(Self::Divide(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            5 => Ok(Self::Double(Decodable::decode(&mut d)?)),
            6 => Ok(Self::Square(Decodable::decode(&mut d)?)),
            7 => Ok(Self::Invert(Decodable::decode(&mut d)?)),
            8 => Ok(Self::UnpackBits(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            9 => Ok(Self::Local),
            10 => Ok(Self::Load(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            11 => Ok(Self::Debug(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            12 => Ok(Self::DumpAlloc),
            13 => Ok(Self::DumpLocal),
            i => Err(Error::BadOperationType),
        }
    }
}

impl_vec!(CryptoOperation);

impl Encodable for ConstraintInstruction {
    fn encode<S: io::Write>(&self, mut s: S) -> Result<usize> {
        unimplemented!();
        Ok(0)
    }
}

impl Decodable for ConstraintInstruction {
    fn decode<D: io::Read>(mut d: D) -> Result<Self> {
        let constraint_type = ReadExt::read_u8(&mut d)?;
        match constraint_type {
            0 => Ok(Self::Lc0Add(Decodable::decode(&mut d)?)),
            1 => Ok(Self::Lc1Add(Decodable::decode(&mut d)?)),
            2 => Ok(Self::Lc2Add(Decodable::decode(&mut d)?)),
            3 => Ok(Self::Lc0Sub(Decodable::decode(&mut d)?)),
            4 => Ok(Self::Lc1Sub(Decodable::decode(&mut d)?)),
            5 => Ok(Self::Lc2Sub(Decodable::decode(&mut d)?)),
            6 => Ok(Self::Lc0AddOne),
            7 => Ok(Self::Lc1AddOne),
            8 => Ok(Self::Lc2AddOne),
            9 => Ok(Self::Lc0SubOne),
            10 => Ok(Self::Lc1SubOne),
            11 => Ok(Self::Lc2SubOne),
            12 => Ok(Self::Lc0AddCoeff(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            13 => Ok(Self::Lc1AddCoeff(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            14 => Ok(Self::Lc2AddCoeff(
                Decodable::decode(&mut d)?,
                Decodable::decode(&mut d)?,
            )),
            15 => Ok(Self::Lc0AddOneCoeff(Decodable::decode(&mut d)?)),
            16 => Ok(Self::Lc1AddOneCoeff(Decodable::decode(&mut d)?)),
            17 => Ok(Self::Lc2AddOneCoeff(Decodable::decode(&mut d)?)),
            18 => Ok(Self::Enforce),
            19 => Ok(Self::LcCoeffReset),
            20 => Ok(Self::LcCoeffDouble),
            _ => Err(Error::BadConstraintType),
        }
    }
}

impl_vec!(ConstraintInstruction);
