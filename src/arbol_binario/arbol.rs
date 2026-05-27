use std::cmp::Ordering;
use crate::arbol_binario::nodo::Nodo;

pub struct Arbol<T> {
    pub raiz: Option<Box<Nodo<T>>>,
}

impl<T: std::fmt::Debug> Arbol<T> {
    pub fn nuevo() -> Self {
    Arbol { raiz: None }
    }
    pub fn in_orden(&self) {
        if let Some(ref raiz) = self.raiz {
            raiz.in_orden();
        }
    }
    pub fn insertar_ordenado<F>(&mut self, valor: T, cmp: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        fn insertar_rec<T, F>(nodo: &mut Box<Nodo<T>>, valor: T, cmp: &F)
        where
            F: Fn(&T, &T) -> Ordering,
        {
            if cmp(&valor, &nodo.valor) == Ordering::Less {
                if let Some(ref mut izq) = nodo.izquierda {
                    insertar_rec(izq, valor, cmp);
                } else {
                    nodo.izquierda = Some(Box::new(Nodo {
                        valor,
                        izquierda: None,
                        derecha: None,
                    }));
                }
            } else {
                if let Some(ref mut der) = nodo.derecha {
                    insertar_rec(der, valor, cmp);
                } else {
                    nodo.derecha = Some(Box::new(Nodo {
                        valor,
                        izquierda: None,
                        derecha: None,
                    }));
                }
            }
        }
        if self.raiz.is_none() {
            self.raiz = Some(Box::new(Nodo {
                valor,
                izquierda: None,
                derecha: None,
            }));
        } else {
            insertar_rec(self.raiz.as_mut().unwrap(), valor, &cmp);
        }
    }


}