pub use crate::sync::mpsc::{RecvError, RecvTimeoutError, SendError, TryRecvError, TrySendError};
use crate::{error, fmt};

/// An error returned from the [`send_timeout`] method.
///
/// The error contains the message being sent so it can be recovered.
///
/// [`send_timeout`]: super::Sender::send_timeout
///
/// <div lang="ar" dir="rtl">
///
/// خطأ ترجعه الدالة `send_timeout`.
///
/// </div>
#[derive(PartialEq, Eq, Clone, Copy)]
#[unstable(feature = "mpmc_channel", issue = "126840")]
pub enum SendTimeoutError<T> {
    /// The message could not be sent because the channel is full and the operation timed out.
    ///
    /// If this is a zero-capacity channel, then the error indicates that there was no receiver
    /// available to receive the message and the operation timed out.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// تعذّر إرسال الرسالة لأن القناة ممتلئة وانتهت المهلة.
    ///
    /// </div>
    Timeout(T),

    /// The message could not be sent because the channel is disconnected.
    ///
    /// <div lang="ar" dir="rtl">
    ///
    /// تعذّر إرسال الرسالة لأن القناة مقطوعة.
    ///
    /// </div>
    Disconnected(T),
}

#[unstable(feature = "mpmc_channel", issue = "126840")]
impl<T> fmt::Debug for SendTimeoutError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "SendTimeoutError(..)".fmt(f)
    }
}

#[unstable(feature = "mpmc_channel", issue = "126840")]
impl<T> fmt::Display for SendTimeoutError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SendTimeoutError::Timeout(..) => "timed out waiting on send operation".fmt(f),
            SendTimeoutError::Disconnected(..) => "sending on a disconnected channel".fmt(f),
        }
    }
}

#[unstable(feature = "mpmc_channel", issue = "126840")]
impl<T> error::Error for SendTimeoutError<T> {}

#[unstable(feature = "mpmc_channel", issue = "126840")]
impl<T> From<SendError<T>> for SendTimeoutError<T> {
    fn from(err: SendError<T>) -> SendTimeoutError<T> {
        match err {
            SendError(e) => SendTimeoutError::Disconnected(e),
        }
    }
}
