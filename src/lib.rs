#![deny(clippy::all)]

use napi::Result;
use napi_derive::napi;

/// Gathers the human readable device name of each output device detected
#[napi]
pub fn get_sound_devices() -> Result<Vec<String>, &'static str> {
  cpvc::safe::get_sound_devices().map_err(to_napi_error)
}

/// Get whether the system volume is muted
#[napi]
pub fn get_mute() -> Result<bool, &'static str> {
  cpvc::safe::get_mute().map_err(to_napi_error)
}

/// Gathers the current volume in percent of the default output device
#[napi]
pub fn get_system_volume() -> Result<u8, &'static str> {
  cpvc::safe::get_system_volume().map_err(to_napi_error)
}

/// Mute or unmute the system volume
#[napi]
pub fn set_mute(mute: bool) -> Result<(), &'static str> {
  cpvc::safe::set_mute(mute).map_err(to_napi_error)
}

/// Sets the current volume in percent of the default output device.
/// The `percent` argument should be a number between 0-100.
///
/// # On macOS
/// `cpvc` needs to mute and unmute the audio device to get the hardware device volume to sync
#[napi]
pub fn set_system_volume(percent: u8) -> Result<(), &'static str> {
  if percent > 100 {
    Err(napi::Error::new(
      "BAD_ARGUMENT",
      "percent should be a percentage (0-100)",
    ))
  } else {
    cpvc::safe::set_system_volume(percent).map_err(to_napi_error)
  }
}

fn to_napi_error(err: cpvc::error::Error) -> napi::Error<&'static str> {
  match err {
    cpvc::error::Error::DeviceNotFound => napi::Error::new("DEVICE_NOT_FOUND", "device not found"),
    cpvc::error::Error::DeviceAccessFailed(reason) => {
      napi::Error::new("DEVICE_ACCESS_FAILED", reason)
    }
    cpvc::error::Error::DeviceEnumerationFailed(reason) => {
      napi::Error::new("DEVICE_ENUMERATION_FAILED", reason)
    }
    cpvc::error::Error::VolumeCaptureFailed(reason) => {
      napi::Error::new("VOLUME_CAPTURE_FAILED", reason)
    }
    cpvc::error::Error::VolumeSetFailed(reason) => napi::Error::new("VOLUME_SET_FAILED", reason),
    cpvc::error::Error::MuteSetFailed(reason) => napi::Error::new("MUTE_SET_FAILED", reason),
    cpvc::error::Error::PlatformUnsupported => {
      napi::Error::new("PLATFORM_UNSUPPORTED", "this platform is not supported")
    }
    _ => napi::Error::new("UNKNOWN", "unknown error"),
  }
}
