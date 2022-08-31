mod diagnostic;
mod error;
pub use diagnostic::*;
pub use error::*;
pub mod emitter;

pub type Result<T> = std::result::Result<T, Error>;

/// A helper struct for change logic from
/// return something to something with diagnostics array
#[derive(Debug)]
pub struct TWithDiagnosticArray<T: std::fmt::Debug> {
  pub inner: T,
  pub diagnostic: Vec<Diagnostic>,
}

impl<T: std::fmt::Debug> TWithDiagnosticArray<T> {
  pub fn new(inner: T, diagnostic: Vec<Diagnostic>) -> Self {
    Self { inner, diagnostic }
  }

  pub fn diagnostics(&self) -> &Vec<Diagnostic> {
    &self.diagnostic
  }

  pub fn take_inner(self) -> T {
    self.inner
  }
}

// Helper trait to make `TWithDiagnosticArray` convertion more easily.
pub trait IntoTWithDiagnosticArray {
  fn with_diagnostic(self, diagnostic: Vec<Diagnostic>) -> TWithDiagnosticArray<Self>
  where
    Self: Sized + std::fmt::Debug;

  fn with_empty_diagnostic(self) -> TWithDiagnosticArray<Self>
  where
    Self: Sized + std::fmt::Debug,
  {
    TWithDiagnosticArray {
      inner: self,
      diagnostic: vec![],
    }
  }
}

impl<T: Sized + std::fmt::Debug> IntoTWithDiagnosticArray for T {
  fn with_diagnostic(self, diagnostic: Vec<Diagnostic>) -> TWithDiagnosticArray<Self>
  where
    Self: Sized + std::fmt::Debug,
  {
    TWithDiagnosticArray {
      inner: self,
      diagnostic,
    }
  }
}