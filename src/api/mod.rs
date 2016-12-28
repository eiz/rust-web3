//! `Web3` implementation

pub mod eth;
pub mod web3;

use {Transport};

/// Common API for all namespaces
pub trait Namespace<'a, T: Transport + 'a> {
  /// Creates new API namespace
  fn new(transport: &'a T) -> Self where Self: Sized;
}

/// `Web3` wrapper for all namespaces
pub struct Web3<T: Transport> {
  transport: T,
}

impl<T: Transport> Web3<T> {
  /// Create new `Web3` with given transport
  pub fn new(transport: T) -> Self {
    Web3 {
      transport: transport,
    }
  }

  /// Access methods from custom namespace
  pub fn api<'a, A: Namespace<'a, T>>(&'a self) -> A {
    A::new(&self.transport)
  }

  /// Access methods from `eth` namespace
  pub fn eth(&self) -> eth::Eth<T> {
    self.api()
  }

  /// Access methods from `web3` namespace
  pub fn web3(&self) -> web3::Web3<T> {
    self.api()
  }

  /// Access filter methods from `eth` namespace
  pub fn eth_filter(&self) -> ! {
    unimplemented!()
  }
}