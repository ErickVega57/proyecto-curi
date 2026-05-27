#[derive(Clone)]
pub struct Nodo<T> {
    pub valor: T,
    pub izquierda: Option<Box<Nodo<T>>>,
    pub derecha: Option<Box<Nodo<T>>>,
}
impl<T: std::fmt::Debug> Nodo<T> {
    pub fn in_orden(&self){
        if !self.izquierda.is_none() {
            self.izquierda.as_ref().map(|nodo| nodo.in_orden());
        }

        println!("{:?}", self.valor);

        if !self.derecha.is_none() {
            self.derecha.as_ref().map(|nodo| nodo.in_orden());
        }
    }

}