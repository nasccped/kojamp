pub trait KojampOutput<ImplType, ReturnType> {
    fn new(default: ImplType) -> ImplType;
    fn update(&mut self, new_value: ImplType);
    fn log_value(&self);
    fn get_value(&self) -> Result<ReturnType, ReturnType>;
}
