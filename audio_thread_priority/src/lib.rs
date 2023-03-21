use std::error::Error;
use std::fmt;

/// The OS-specific issue is available as `inner`
#[derive(Debug)]
pub struct AudioThreadPriorityError {
    message: String,
    inner: Option<Box<dyn Error + 'static>>,
}

impl AudioThreadPriorityError {
    fn new_with_inner(message: &str, inner: Box<dyn Error>) -> AudioThreadPriorityError {
        AudioThreadPriorityError {
            message: message.into(),
            inner: Some(inner),
        }
    }
    fn new(message: &str) -> AudioThreadPriorityError {
        AudioThreadPriorityError {
            message: message.into(),
            inner: None,
        }
    }
}

impl fmt::Display for AudioThreadPriorityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rv = write!(f, "AudioThreadPriorityError: {}", &self.message);
        if let Some(inner) = &self.inner {
            rv = write!(f, " ({})", inner);
        }
        rv
    }
}

impl Error for AudioThreadPriorityError {
    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.as_ref().map(|e| e.as_ref())
    }
}

mod rt_linux;
extern crate dbus;
extern crate libc;
use rt_linux::demote_thread_from_real_time_internal;
use rt_linux::promote_thread_to_real_time_internal;
use rt_linux::RtPriorityHandleInternal;
use rt_linux::RtPriorityThreadInfoInternal;
pub type RtPriorityHandle = RtPriorityHandleInternal;

/// Opaque handle to a thread info.
///
/// This can be serialized to raw bytes to be sent via IPC.
///
/// This call is useful on Linux desktop only, when the process is sandboxed and
/// cannot promote itself directly.
pub type RtPriorityThreadInfo = RtPriorityThreadInfoInternal;

/// Promote a particular thread thread to real-time priority.
///
/// This call is useful on Linux desktop only, when the process is sandboxed and
/// cannot promote itself directly.
///
/// # Arguments
///
/// * `thread_info` - informations about the thread to promote, gathered using
/// `get_current_thread_info`.
/// * `audio_buffer_frames` - the exact or an upper limit on the number of frames that have to be
/// rendered each callback, or 0 for a sensible default value.
/// * `audio_samplerate_hz` - the sample-rate for this audio stream, in Hz.
///
/// # Return value
///
/// This function returns a `Result<RtPriorityHandle>`, which is an opaque struct to be passed to
/// `demote_current_thread_from_real_time` to revert to the previous thread priority.
pub fn promote_thread_to_real_time(
    thread_info: RtPriorityThreadInfo,
    audio_buffer_frames: u32,
    audio_samplerate_hz: u32,
) -> Result<RtPriorityHandle, AudioThreadPriorityError> {
    if audio_samplerate_hz == 0 {
        return Err(AudioThreadPriorityError::new("sample rate is zero"));
    }
    promote_thread_to_real_time_internal(thread_info, audio_buffer_frames, audio_samplerate_hz)
}

/// Demotes a thread from real-time priority.
///
/// # Arguments
///
/// * `thread_info` - An opaque struct returned from a successful call to
/// `get_current_thread_info`.
///
/// # Return value
///
/// `Ok` in case of success, `Err` otherwise.
pub fn demote_thread_from_real_time(
    thread_info: RtPriorityThreadInfo,
) -> Result<(), AudioThreadPriorityError> {
    demote_thread_from_real_time_internal(thread_info)
}
