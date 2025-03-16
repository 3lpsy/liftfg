pub trait Includable {}

// TODO validate max length on includes
pub trait HasIncludes<T>
where
    T: Includable,
{
    fn includes(&mut self) -> &mut Option<Vec<T>>;
    fn with_include(mut self, include: T) -> Self
    where
        Self: Sized,
    {
        let includes = self.includes();
        if includes.is_none() {
            *includes = Some(Vec::new());
        }
        includes.as_mut().unwrap().push(include);
        self
    }
}
