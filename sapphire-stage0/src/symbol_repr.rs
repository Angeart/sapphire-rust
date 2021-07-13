use derivative::Derivative;
use derive_new::new;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct SymbolRepl<T> {
  pub id: Uuid,
  #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
  _marker: PhantomData<fn() -> T>,
}

impl<T> Copy for SymbolRepl<T> {}

impl<T> Clone for SymbolRepl<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Hash for SymbolRepl<T> {
  fn hash<H>(&self, state: &mut H)
  where
    H: std::hash::Hasher,
  {
    self.id.hash(state)
  }
}

impl<T> SymbolRepl<T> {
  pub fn new() -> Self {
    SymbolRepl {
      id: Uuid::new_v4(),
      _marker: PhantomData,
    }
  }
}

#[derive(Debug, Clone, new)]
pub struct SymbolReplTable<T: Hash + Eq> {
  symbols: HashMap<T, SymbolRepl<T>>,
  refs: HashMap<Uuid, T>,
}

impl<T: Hash + Eq> SymbolReplTable<T> {
  pub fn get<Q: ?Sized>(&self, id: &Q) -> Option<SymbolRepl<T>>
  where
    Q: Hash + Eq,
    T: Borrow<Q>,
  {
    self.symbols.get(id).copied()
  }

  pub fn has<Q: ?Sized>(&self, id: &Q) -> bool
  where
    Q: Hash + Eq,
    T: Borrow<Q>,
  {
    self.symbols.get(id).is_some()
  }

  pub fn create<Q: ?Sized>(&mut self, id: &Q) -> SymbolRepl<T>
  where
    Q: Hash + Eq + ToOwned<Owned = T>,
    T: Borrow<Q>,
  {
    match self.get(id) {
      Some(symbol) => symbol,
      None => {
        let symbol = SymbolRepl::new();
        self.refs.insert(symbol.id.to_owned(), id.to_owned());
        self.symbols.insert(id.to_owned(), symbol);
        symbol
      }
    }
  }
  pub fn entity(&self, symbol: SymbolRepl<T>) -> &T {
    &self.refs.get(&symbol.id).unwrap()
  }
}
