use libc::{c_char, c_int};

/// Response status codes.
pub type SquashStatus = c_int;

/// Operation completed successfully.
pub const SQUASH_OK: SquashStatus                    =  1;
/// Operation partially completed.
pub const SQUASH_PROCESSING: SquashStatus            =  2;
/// Reached the end of the stream while decoding.
pub const SQUASH_END_OF_STREAM: SquashStatus         =  3;
/// Operation failed.
pub const SQUASH_FAILED: SquashStatus                = -1;
/// Unable to load the requested resource.
pub const SQUASH_UNABLE_TO_LOAD: SquashStatus        = -2;
/// One or more of the parameters were not valid.
pub const SQUASH_BAD_PARAM: SquashStatus             = -3;
/// One or more parameter values was not valid.
pub const SQUASH_BAD_VALUE: SquashStatus             = -4;
/// Not enough memory is available.
pub const SQUASH_MEMORY: SquashStatus                = -5;
/// Insufficient space in buffer.
pub const SQUASH_BUFFER_FULL: SquashStatus           = -6;
/// Supplied buffer was empty.
pub const SQUASH_BUFFER_EMPTY: SquashStatus          = -7;
/// Performing the requested operation from the current state is not supported.
pub const SQUASH_STATE: SquashStatus                 = -8;
/// The requested operation is not available.
pub const SQUASH_INVALID_OPERATION: SquashStatus     = -9;
/// The requested codec could not be found.
pub const SQUASH_NOT_FOUND: SquashStatus             = -10;
/// A buffer passed to squash was invalid.
pub const SQUASH_INVALID_BUFFER: SquashStatus        = -11;
/// An input/output error occurred.
///
/// There is a good chance errno will have additional details, though it
/// is not guaranteed.
pub const SQUASH_IO: SquashStatus                    = -12;
/// A buffer was too large to be usable.
///
/// While Squash uses size_t for buffer sizes, not all libraries used by
/// plugins do. Many use int, long, unsigned int, etc., which may be smaller
/// than size_t (or, less likely, larger).
///
/// In the event that converting a value between representations is not
/// possible, `SQUASH_RANGE` will be returned.
pub const SQUASH_RANGE: SquashStatus                 = -13;

extern {
    /// Get a string representation of a status code.
    ///
    /// # Parameters
    /// * `status` The status.
    ///
    /// # Returns
    /// A string describing status
    pub fn squash_status_to_string(status: SquashStatus) -> *const c_char;

    /// Emit an error.
    ///
    /// This function simply returns the argument which was passed to it. It
    /// exists only to make it easier to debug an error by setting a
    /// breakpoint on this function.
    ///
    /// Note that only the initial point at which the error was generated
    /// should call this function. Do not call it when simply returning an
    /// error that was generated by another function.
    ///
    /// # Parameters
    /// * `status` the error
    ///
    /// # Returns
    /// the error
    pub fn squash_error(status: SquashStatus) -> SquashStatus;
}
