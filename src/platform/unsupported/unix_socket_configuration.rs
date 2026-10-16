//! Stand-ins for `UnixSocketConfiguration.swift`.

use super::CzOutcome;
use super::SocketDirection;
use std::convert::Infallible;

taken!(unix_socket_configuration -> CzUnixSocketConfiguration);

handles!(CzUnixSocketConfiguration);

failing!(cz_unix_socket_configuration_new(&str, &str));

impl CzUnixSocketConfiguration {
  pub(crate) fn duplicate(&self) -> CzUnixSocketConfiguration {
    match self.0 {}
  }

  pub(crate) fn id(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn source(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn destination(&self) -> String {
    match self.0 {}
  }

  pub(crate) fn permissions(&self) -> Option<u16> {
    match self.0 {}
  }

  pub(crate) fn direction(&self) -> SocketDirection {
    match self.0 {}
  }

  pub(crate) fn set_source(&self, _source: &str) {
    match self.0 {}
  }

  pub(crate) fn set_destination(&self, _destination: &str) {
    match self.0 {}
  }

  pub(crate) fn set_permissions(&self, _permissions: Option<u16>) {
    match self.0 {}
  }

  pub(crate) fn set_direction(&self, _direction: SocketDirection) {
    match self.0 {}
  }
}
