//! Re-export the `http2` module for HTTP/2 frame types and utilities.

use http2::frame::ExperimentalSettings;
pub use http2::frame::{
    Priorities, PrioritiesBuilder, Priority, PseudoId, PseudoOrder, Setting, SettingId,
    SettingsOrder, SettingsOrderBuilder, StreamDependency, StreamId,
};

use crate::core::proto::{
    h2::client::Config,
    {self},
};

/// Builder for `Http2Config`.
#[must_use]
#[derive(Debug)]
pub struct Http2ConfigBuilder {
    config: Http2Config,
}

/// Configuration config for an HTTP/2 connection.
///
/// This struct defines various parameters to fine-tune the behavior of an HTTP/2 connection,
/// including stream management, window sizes, frame limits, and header config.
#[derive(Debug, Clone, Default)]
pub struct Http2Config {
    pub(crate) h2_builder: Config,
}

impl Http2ConfigBuilder {
    /// Sets the [`SETTINGS_INITIAL_WINDOW_SIZE`][spec] option for HTTP2
    /// stream-level flow control.
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, crate::core: will use a default.
    ///
    /// [spec]: https://httpwg.org/specs/rfc9113.html#SETTINGS_INITIAL_WINDOW_SIZE
    pub fn initial_stream_window_size(mut self, sz: impl Into<Option<u32>>) -> Self {
        if let Some(sz) = sz.into() {
            self.config.h2_builder.adaptive_window = false;
            self.config.h2_builder.initial_stream_window_size = sz;
        }
        self
    }

    /// Sets the max connection-level flow control for HTTP2
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, crate::core: will use a default.
    pub fn initial_connection_window_size(mut self, sz: impl Into<Option<u32>>) -> Self {
        if let Some(sz) = sz.into() {
            self.config.h2_builder.adaptive_window = false;
            self.config.h2_builder.initial_conn_window_size = sz;
        }
        self
    }

    /// Sets the initial maximum of locally initiated (send) streams.
    ///
    /// This value will be overwritten by the value included in the initial
    /// SETTINGS frame received from the peer as part of a [connection preface].
    ///
    /// Passing `None` will do nothing.
    ///
    /// If not set, crate::core: will use a default.
    ///
    /// [connection preface]: https://httpwg.org/specs/rfc9113.html#preface
    pub fn initial_max_send_streams(mut self, initial: impl Into<Option<usize>>) -> Self {
        if let Some(initial) = initial.into() {
            self.config.h2_builder.initial_max_send_streams = initial;
        }
        self
    }

    /// Sets the initial stream id for the connection.
    pub fn initial_stream_id(mut self, id: impl Into<Option<u32>>) -> Self {
        self.config.h2_builder.initial_stream_id = id.into();
        self
    }

    /// Sets whether to use an adaptive flow control.
    ///
    /// Enabling this will override the limits set in
    /// `initial_stream_window_size` and
    /// `initial_connection_window_size`.
    pub fn adaptive_window(mut self, enabled: bool) -> Self {
        use proto::h2::SPEC_WINDOW_SIZE;

        self.config.h2_builder.adaptive_window = enabled;
        if enabled {
            self.config.h2_builder.initial_conn_window_size = SPEC_WINDOW_SIZE;
            self.config.h2_builder.initial_stream_window_size = SPEC_WINDOW_SIZE;
        }
        self
    }

    /// Sets the maximum frame size to use for HTTP2.
    ///
    /// Default is currently 16KB, but can change.
    pub fn max_frame_size(mut self, sz: impl Into<Option<u32>>) -> Self {
        self.config.h2_builder.max_frame_size = sz.into();
        self
    }

    /// Sets the max size of received header frames.
    ///
    /// Default is currently 16KB, but can change.
    pub fn max_header_list_size(mut self, max: u32) -> Self {
        self.config.h2_builder.max_header_list_size = Some(max);
        self
    }

    /// Sets the header table size.
    ///
    /// This setting informs the peer of the maximum size of the header compression
    /// table used to encode header blocks, in octets. The encoder may select any value
    /// equal to or less than the header table size specified by the sender.
    ///
    /// The default value of crate `h2` is 4,096.
    pub fn header_table_size(mut self, size: impl Into<Option<u32>>) -> Self {
        self.config.h2_builder.header_table_size = size.into();
        self
    }

    /// Sets the maximum number of concurrent streams.
    ///
    /// The maximum concurrent streams setting only controls the maximum number
    /// of streams that can be initiated by the remote peer. In other words,
    /// when this setting is set to 100, this does not limit the number of
    /// concurrent streams that can be created by the caller.
    ///
    /// It is recommended that this value be no smaller than 100, so as to not
    /// unnecessarily limit parallelism. However, any value is legal, including
    /// 0. If `max` is set to 0, then the remote will not be permitted to
    /// initiate streams.
    ///
    /// Note that streams in the reserved state, i.e., push promises that have
    /// been reserved but the stream has not started, do not count against this
    /// setting.
    ///
    /// Also note that if the remote *does* exceed the value set here, it is not
    /// a protocol level error. Instead, the `h2` library will immediately reset
    /// the stream.
    ///
    /// See [Section 5.1.2] in the HTTP/2 spec for more details.
    ///
    /// [Section 5.1.2]: https://http2.github.io/http2-spec/#rfc.section.5.1.2
    pub fn max_concurrent_streams(mut self, max: impl Into<Option<u32>>) -> Self {
        self.config.h2_builder.max_concurrent_streams = max.into();
        self
    }

    /// Enables and disables the push feature for HTTP2.
    ///
    /// Passing `None` will do nothing.
    pub fn enable_push(mut self, opt: bool) -> Self {
        self.config.h2_builder.enable_push = Some(opt);
        self
    }

    /// Sets the enable connect protocol.
    pub fn enable_connect_protocol(mut self, opt: bool) -> Self {
        self.config.h2_builder.enable_connect_protocol = Some(opt);
        self
    }

    /// Disable RFC 7540 Stream Priorities (set to `true` to disable).
    /// [RFC 9218]: <https://www.rfc-editor.org/rfc/rfc9218.html#section-2.1>
    pub fn no_rfc7540_priorities(mut self, opt: bool) -> Self {
        self.config.h2_builder.no_rfc7540_priorities = Some(opt);
        self
    }

    /// Sets the maximum number of HTTP2 concurrent locally reset streams.
    ///
    /// See the documentation of [`http2::client::Builder::max_concurrent_reset_streams`] for more
    /// details.
    ///
    /// The default value is determined by the `h2` crate.
    ///
    /// [`http2::client::Builder::max_concurrent_reset_streams`]: https://docs.rs/h2/client/struct.Builder.html#method.max_concurrent_reset_streams
    pub fn max_concurrent_reset_streams(mut self, max: usize) -> Self {
        self.config.h2_builder.max_concurrent_reset_streams = Some(max);
        self
    }

    /// Set the maximum write buffer size for each HTTP/2 stream.
    ///
    /// Default is currently 1MB, but may change.
    ///
    /// # Panics
    ///
    /// The value must be no larger than `u32::MAX`.
    pub fn max_send_buf_size(mut self, max: usize) -> Self {
        assert!(max <= u32::MAX as usize);
        self.config.h2_builder.max_send_buffer_size = max;
        self
    }

    /// Configures the maximum number of pending reset streams allowed before a GOAWAY will be sent.
    ///
    /// See <https://github.com/hyperium/hyper/issues/2877> for more information.
    pub fn max_pending_accept_reset_streams(mut self, max: impl Into<Option<usize>>) -> Self {
        self.config.h2_builder.max_pending_accept_reset_streams = max.into();
        self
    }

    /// Sets the stream dependency and weight for the outgoing HEADERS frame.
    ///
    /// This configures the priority of the stream by specifying its dependency and weight,
    /// as defined by the HTTP/2 priority mechanism. This can be used to influence how the
    /// server allocates resources to this stream relative to others.
    pub fn headers_stream_dependency<T>(mut self, stream_dependency: T) -> Self
    where
        T: Into<Option<StreamDependency>>,
    {
        self.config.h2_builder.headers_stream_dependency = stream_dependency.into();
        self
    }

    /// Sets the HTTP/2 pseudo-header field order for outgoing HEADERS frames.
    ///
    /// This determines the order in which pseudo-header fields (such as `:method`, `:scheme`, etc.)
    /// are encoded in the HEADERS frame. Customizing the order may be useful for interoperability
    /// or testing purposes.
    pub fn headers_pseudo_order<T>(mut self, headers_pseudo_order: T) -> Self
    where
        T: Into<Option<PseudoOrder>>,
    {
        self.config.h2_builder.headers_pseudo_order = headers_pseudo_order.into();
        self
    }

    /// Configures custom experimental HTTP/2 setting.
    ///
    /// This setting is reserved for future use or experimental purposes.
    /// Enabling or disabling it may have no effect unless explicitly supported
    /// by the server or client implementation.
    pub fn experimental_settings<T>(mut self, experimental_settings: T) -> Self
    where
        T: Into<Option<ExperimentalSettings>>,
    {
        self.config.h2_builder.experimental_settings = experimental_settings.into();
        self
    }

    /// Sets the order of settings parameters in the initial SETTINGS frame.
    ///
    /// This determines the order in which settings are sent during the HTTP/2 handshake.
    /// Customizing the order may be useful for testing or protocol compliance.
    pub fn settings_order<T>(mut self, settings_order: T) -> Self
    where
        T: Into<Option<SettingsOrder>>,
    {
        self.config.h2_builder.settings_order = settings_order.into();
        self
    }

    /// Sets the list of PRIORITY frames to be sent immediately after the connection is established,
    /// but before the first request is sent.
    ///
    /// This allows you to pre-configure the HTTP/2 stream dependency tree by specifying a set of
    /// PRIORITY frames that will be sent as part of the connection preface. This can be useful for
    /// optimizing resource allocation or testing custom stream prioritization strategies.
    ///
    /// Each `Priority` in the list must have a valid (non-zero) stream ID. Any priority with a
    /// stream ID of zero will be ignored.
    pub fn priorities<T>(mut self, priorities: T) -> Self
    where
        T: Into<Option<Priorities>>,
    {
        self.config.h2_builder.priorities = priorities.into();
        self
    }

    /// Builds the `Http2Config` instance.
    pub fn build(self) -> Http2Config {
        self.config
    }
}

impl Http2Config {
    /// Creates a new `Http2ConfigBuilder` instance.
    pub fn builder() -> Http2ConfigBuilder {
        Http2ConfigBuilder {
            config: Http2Config::default(),
        }
    }
}
