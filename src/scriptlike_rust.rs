use core::marker::PhantomData;

struct ScriptlikeRust<'a, T>
where
    T: AsRef<&'a str>
{
    source: T,
    phantom: PhantomData<&'a ()>,
}