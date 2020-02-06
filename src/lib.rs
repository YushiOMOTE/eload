use log::*;
use serde::{de::DeserializeOwned, ser, Serialize};
use serde_yaml::Value;
use std::collections::HashSet;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unpack error {}: {}", _0, _1)]
    UnpackError(String, String),
    #[error("Pack error: {}", _0)]
    PackError(String),
    #[error("Unsupported")]
    Unsupported,
    #[error("Invalid unicode: {}", _0)]
    VarError(String),
}

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::PackError(msg.to_string())
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::PackError(e.to_string())
    }
}

type Result<T> = std::result::Result<T, Error>;

fn to_key_str(key: &Value) -> String {
    match key {
        Value::String(s) => s.to_uppercase(),
        e => unreachable!("Key must be string. Found: {:?}", e),
    }
}

fn find_and_update(value: &mut Value, cur: &str, target: &str, new_value: &Value) -> bool {
    if cur == target {
        *value = new_value.clone();
        return true;
    }

    match value {
        Value::Mapping(map) => {
            for (key, mut value) in map {
                let key = to_key_str(&key);

                if find_and_update(
                    &mut value,
                    &(cur.to_owned() + "_" + &key),
                    target,
                    new_value,
                ) {
                    return true;
                }
            }

            false
        }
        _ => false,
    }
}

pub struct Serializer {
    curpath: Vec<String>,
    paths: HashSet<String>,
    value: Value,
}

impl Serializer {
    fn new(prefix: &str, value: Value) -> Self {
        Self {
            curpath: vec![prefix.to_uppercase()],
            paths: HashSet::new(),
            value,
        }
    }

    fn enter(&mut self, name: &str) {
        self.curpath.push(name.to_uppercase());
    }

    fn exit(&mut self) {
        self.curpath.pop();
    }

    fn path(&self) -> String {
        self.curpath.join("_")
    }

    fn load(&mut self) -> Result<()> {
        let path = self.path();

        if !self.paths.insert(path.clone()) {
            warn!("warning: environment variable {} is ambiguous", path);
        }

        match std::env::var(&path) {
            Ok(val) => {
                let val = if val.is_empty() { "~".into() } else { val };
                let val = serde_yaml::from_str(&val)?;
                let target = self.path().clone();
                let prefix = self.curpath[0].clone();
                find_and_update(&mut self.value, &prefix, &target, &val);
                Ok(())
            }
            Err(std::env::VarError::NotPresent) => Ok(()),
            Err(e) => Err(Error::VarError(e.to_string())),
        }
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _: bool) -> Result<()> {
        self.load()
    }

    fn serialize_i8(self, _: i8) -> Result<()> {
        self.load()
    }

    fn serialize_i16(self, _: i16) -> Result<()> {
        self.load()
    }

    fn serialize_i32(self, _: i32) -> Result<()> {
        self.load()
    }

    fn serialize_i64(self, _: i64) -> Result<()> {
        self.load()
    }

    fn serialize_u8(self, _: u8) -> Result<()> {
        self.load()
    }

    fn serialize_u16(self, _: u16) -> Result<()> {
        self.load()
    }

    fn serialize_u32(self, _: u32) -> Result<()> {
        self.load()
    }

    fn serialize_u64(self, _: u64) -> Result<()> {
        self.load()
    }

    fn serialize_f32(self, _: f32) -> Result<()> {
        self.load()
    }

    fn serialize_f64(self, _: f64) -> Result<()> {
        self.load()
    }

    fn serialize_char(self, _: char) -> Result<()> {
        self.load()
    }

    fn serialize_str(self, _: &str) -> Result<()> {
        self.load()
    }

    fn serialize_bytes(self, _: &[u8]) -> Result<()> {
        self.load()
    }

    fn serialize_none(self) -> Result<()> {
        self.load()
    }

    fn serialize_some<T>(self, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.load()
    }

    fn serialize_unit(self) -> Result<()> {
        self.load()
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<()> {
        self.load()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.load()
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, _: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.load()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.load()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.load()?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.load()?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.load()?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.load()?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.load()?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.load()?;
        Ok(self)
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.enter(key);
        value.serialize(&mut **self)?;
        self.exit();
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

pub fn load<T: Serialize + DeserializeOwned>(pfx: &str, t: &T) -> Result<T> {
    let value = serde_yaml::to_value(&t)?;
    let mut ser = Serializer::new(pfx, value);
    t.serialize(&mut ser)?;
    Ok(serde_yaml::from_value(ser.value)?)
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Deserialize;

    struct Vars {
        varset: Vec<String>,
    }

    impl Vars {
        fn new(varset: Vec<(String, String)>) -> Self {
            Self {
                varset: varset
                    .into_iter()
                    .map(|(key, value)| {
                        std::env::set_var(&key, &value);
                        key
                    })
                    .collect(),
            }
        }
    }

    impl Drop for Vars {
        fn drop(&mut self) {
            for key in &self.varset {
                std::env::remove_var(&key);
            }
        }
    }

    macro_rules! vars {
        ($($k:literal => $v:literal;)*) => {
            Vars::new(vec![$(($k.into(), $v.into())),*])
        }
    }

    #[test]
    fn test_envs_simple() {
        #[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
        struct A {
            a: u32,
            b: u32,
            c: u32,
        }

        let a = A::default();

        let _v = vars!(
            "PFX_A" => "10";
        );
        assert_eq!(load("pfx", &a).unwrap(), A { a: 10, b: 0, c: 0 });
    }

    #[test]
    fn test_envs_nested() {
        #[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
        struct A {
            a: u32,
            b: u32,
            c: B,
        }
        #[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
        struct B {
            a: u32,
            b: C,
            c: u32,
        }
        #[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
        struct C {
            a: u32,
            b: u32,
            c: u32,
        }

        let a = A::default();

        assert_eq!(load("pfx", &a).unwrap(), a);
        let _v = vars!(
            "PFX_B" => "32";
            "PFX_C_A" => "12";
            "PFX_C_B_B" => "42";
        );
        assert_eq!(
            load("pfx", &a).unwrap(),
            A {
                a: 0,
                b: 32,
                c: B {
                    a: 12,
                    b: C { a: 0, b: 42, c: 0 },
                    c: 0
                },
            }
        );
    }

    #[test]
    fn test_types() {
        use std::collections::*;
        use std::net::SocketAddr;
        use std::path::PathBuf;
        use std::time::Duration;

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct A {
            a: bool,
            u1: u8,
            u2: u16,
            u3: u32,
            u4: u64,
            i1: i8,
            i2: i16,
            i3: i32,
            i4: i64,
            f1: f32,
            f2: f64,
            s: String,
            c: char,
            o1: Option<String>,
            o2: Option<String>,
            bt: BTreeSet<String>,
            hm: HashMap<String, String>,
            ll: LinkedList<String>,
            hs: HashSet<String>,
            vd: VecDeque<String>,
            vc: Vec<String>,
            a1: [u32; 1],
            a2: [u32; 3],
            unit: (),
            t1: (u32,),
            t2: (u32, bool),
            t3: (u32, bool, String),
            sock: SocketAddr,
            path: PathBuf,
            dur: Duration,
        }

        let mut a = A {
            a: false,
            u1: 1,
            u2: 2,
            u3: 3,
            u4: 4,
            i1: -1,
            i2: -2,
            i3: -3,
            i4: 4,
            f1: 3.14,
            f2: -3.1415,
            s: "hello".into(),
            c: 'c',
            o1: None,
            o2: Some("world".into()),
            bt: BTreeSet::new(),
            hm: HashMap::new(),
            ll: LinkedList::new(),
            hs: HashSet::new(),
            vd: VecDeque::new(),
            vc: Vec::new(),
            a1: [1],
            a2: [1, 2, 3],
            unit: (),
            t1: (3,),
            t2: (0, false),
            t3: (0, false, "".into()),
            sock: "127.0.0.1:3939".parse().unwrap(),
            path: PathBuf::from("/home/unko"),
            dur: Duration::from_secs(39),
        };

        assert_eq!(load("pfx", &a).unwrap(), a);
        let _v = vars!(
            "PFX_A" => "true";
            "PFX_U1" => "4";
            "PFX_U2" => "5";
            "PFX_U3" => "6";
            "PFX_U4" => "7";
            "PFX_I1" => "-5";
            "PFX_I2" => "-4";
            "PFX_I3" => "-3";
            "PFX_I4" => "-2";
            "PFX_F1" => "1.414";
            "PFX_F2" => "-1.414";
            "PFX_S" => "unko";
            "PFX_C" => "z";
            "PFX_O1" => "Gotcha";
            "PFX_O2" => "";
            "PFX_BT" => "[a,b,c]";
            "PFX_HM" => "{'a': 'b', 'c':'d'}";
            "PFX_LL" => "[u,n,k,o]";
            "PFX_HS" => "[h, s]";
            "PFX_VD" => "[bigg_s, d_ckus]";
            "PFX_VC" => "[incontinentia, buttocks]";
            "PFX_A1" => "[3]";
            "PFX_A2" => "[1, 1, 5]";
            "PFX_UNIT" => "";
            "PFX_T1" => "[8]";
            "PFX_T2" => "[8, true]";
            "PFX_T3" => "[8, true, '6']";
            "PFX_SOCK" => "10.32.0.33:4454";
            "PFX_PATH" => "/home/biggus/dickus";
            "PFX_DUR" => "30sec";
        );
        a.a = true;
        a.u1 = 4;
        a.u2 = 5;
        a.u3 = 6;
        a.u4 = 7;
        a.i1 = -5;
        a.i2 = -4;
        a.i3 = -3;
        a.i4 = -2;
        a.f1 = 1.414;
        a.f2 = -1.414;
        a.s = "unko".into();
        a.c = 'z';
        a.o1 = Some("Gotcha".into());
        a.o2 = None;
        a.bt = vec!["a".into(), "b".into(), "c".into()]
            .into_iter()
            .collect();
        a.hm = vec![("a".into(), "b".into()), ("c".into(), "d".into())]
            .into_iter()
            .collect();
        a.ll = vec!["u".into(), "n".into(), "k".into(), "o".into()]
            .into_iter()
            .collect();
        a.hs = vec!["h".into(), "s".into()].into_iter().collect();
        a.vd = vec!["bigg_s".into(), "d_ckus".into()].into_iter().collect();
        a.vc = vec!["incontinentia".into(), "buttocks".into()]
            .into_iter()
            .collect();
        a.a1 = [3];
        a.a2 = [1, 1, 5];
        a.t1 = (8,);
        a.t2 = (8, true);
        a.t3 = (8, true, "6".into());
        a.sock = "10.32.0.33:4454".parse().unwrap();
        a.path = PathBuf::from("/home/biggus/dickus");
        a.dur = Duration::from_secs(30);
        assert_eq!(load("pfx", &a).unwrap(), a,);
    }

    #[test]
    fn test_envs_mixed() {
        #[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
        struct A {
            a: bool,
            b: u8,
            c: B,
        }
        #[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
        struct B {
            a: Vec<u32>,
            b: HashSet<u32>,
            c: i32,
            d: i8,
        }

        let a = A::default();

        assert_eq!(load("pfx", &a).unwrap(), a);
        let _v = vars!(
            "PFX_A" => "true";
            "PFX_B" => "33";
            "PFX_C_A" => "[1,2,3]";
            "PFX_C_C" => "-1";
            "PFX_C_D" => "-3";
        );
        assert_eq!(
            load("pfx", &a).unwrap(),
            A {
                a: true,
                b: 33,
                c: B {
                    a: vec![1, 2, 3],
                    b: HashSet::new(),
                    c: -1,
                    d: -3,
                }
            }
        );
    }
}
