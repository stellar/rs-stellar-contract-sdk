use core::marker::PhantomData;

use super::{
    xdr::ScObjectType, Env, EnvObj, EnvRawValConvertible, EnvTrait, EnvVal, EnvValConvertible,
    OrAbort, RawVal, Vec,
};

#[repr(transparent)]
#[derive(Clone)]
pub struct Map<K, V>(EnvObj, PhantomData<K>, PhantomData<V>);

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> TryFrom<EnvVal<RawVal>> for Map<K, V> {
    type Error = ();

    #[inline(always)]
    fn try_from(ev: EnvVal<RawVal>) -> Result<Self, Self::Error> {
        let obj: EnvObj = ev.clone().try_into()?;
        obj.try_into()
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> TryFrom<EnvObj> for Map<K, V> {
    type Error = ();

    #[inline(always)]
    fn try_from(obj: EnvObj) -> Result<Self, Self::Error> {
        if obj.as_tagged().is_obj_type(ScObjectType::ScoMap) {
            Ok(Map(obj, PhantomData, PhantomData))
        } else {
            Err(())
        }
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> From<Map<K, V>> for RawVal {
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0.into()
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> From<Map<K, V>> for EnvVal<RawVal> {
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0.into()
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> From<Map<K, V>> for EnvObj {
    #[inline(always)]
    fn from(m: Map<K, V>) -> Self {
        m.0
    }
}

impl<K: EnvRawValConvertible, V: EnvRawValConvertible> Map<K, V> {
    #[inline(always)]
    unsafe fn unchecked_new(obj: EnvObj) -> Self {
        Self(obj, PhantomData, PhantomData)
    }

    #[inline(always)]
    fn env(&self) -> &Env {
        &self.0.env()
    }

    #[inline(always)]
    pub fn new(env: &Env) -> Map<K, V> {
        let obj = env.map_new().in_env(env);
        unsafe { Self::unchecked_new(obj) }
    }

    #[inline(always)]
    pub fn has(&self, k: K) -> bool {
        let env = self.env();
        let has = env.map_has(self.0.to_tagged(), k.into_val(env));
        bool::try_from_val(env, has).or_abort()
    }

    #[inline(always)]
    pub fn get(&self, k: K) -> V {
        let env = self.env();
        let v = env.map_get(self.0.to_tagged(), k.into_val(env));
        V::try_from_val(env, v).or_abort()
    }

    #[inline(always)]
    pub fn put(&mut self, k: K, v: V) {
        let env = self.env();
        let map = env.map_put(self.0.to_tagged(), k.into_val(env), v.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn del(&mut self, k: K) {
        let env = self.env();
        let map = env.map_del(self.0.to_tagged(), k.into_val(env));
        self.0 = map.in_env(env);
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        let env = self.env();
        let len = env.map_len(self.0.to_tagged());
        u32::try_from_val(env, len).or_abort()
    }

    #[inline(always)]
    pub fn keys(&self) -> Vec<K> {
        let env = self.env();
        let vec = env.map_keys(self.0.to_tagged());
        Vec::<K>::try_from_val(env, vec).or_abort()
    }
}
