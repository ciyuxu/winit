use std::os::raw::c_void;

use icrate::Foundation::MainThreadMarker;
use objc2::rc::Id;

use crate::{
    event_loop::EventLoop,
    monitor::{MonitorHandle, VideoMode},
    window::{Window, WindowBuilder},
};

/// Additional methods on [`EventLoop`] that are specific to iOS.
pub trait EventLoopExtIOS {
    /// Returns the [`Idiom`] (phone/tablet/tv/etc) for the current device.
    fn idiom(&self) -> Idiom;
}

impl<T: 'static> EventLoopExtIOS for EventLoop<T> {
    fn idiom(&self) -> Idiom {
        self.event_loop.idiom()
    }
}

/// Additional methods on [`Window`] that are specific to iOS.
pub trait WindowExtIOS {
    /// Sets the [`contentScaleFactor`] of the underlying [`UIWindow`] to `scale_factor`.
    ///
    /// The default value is device dependent, and it's recommended GLES or Metal applications set
    /// this to [`MonitorHandle::scale_factor()`].
    ///
    /// [`UIWindow`]: https://developer.apple.com/documentation/uikit/uiwindow?language=objc
    /// [`contentScaleFactor`]: https://developer.apple.com/documentation/uikit/uiview/1622657-contentscalefactor?language=objc
    fn set_scale_factor(&self, scale_factor: f64);

    /// Sets the valid orientations for the [`Window`].
    ///
    /// The default value is [`ValidOrientations::LandscapeAndPortrait`].
    ///
    /// This changes the value returned by
    /// [`-[UIViewController supportedInterfaceOrientations]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/1621435-supportedinterfaceorientations?language=objc),
    /// and then calls
    /// [`-[UIViewController attemptRotationToDeviceOrientation]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/1621400-attemptrotationtodeviceorientati?language=objc).
    fn set_valid_orientations(&self, valid_orientations: ValidOrientations);

    /// Sets whether the [`Window`] prefers the home indicator hidden.
    ///
    /// The default is to prefer showing the home indicator.
    ///
    /// This changes the value returned by
    /// [`-[UIViewController prefersHomeIndicatorAutoHidden]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/2887510-prefershomeindicatorautohidden?language=objc),
    /// and then calls
    /// [`-[UIViewController setNeedsUpdateOfHomeIndicatorAutoHidden]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/2887509-setneedsupdateofhomeindicatoraut?language=objc).
    ///
    /// This only has an effect on iOS 11.0+.
    fn set_prefers_home_indicator_hidden(&self, hidden: bool);

    /// Sets the screen edges for which the system gestures will take a lower priority than the
    /// application's touch handling.
    ///
    /// This changes the value returned by
    /// [`-[UIViewController preferredScreenEdgesDeferringSystemGestures]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/2887512-preferredscreenedgesdeferringsys?language=objc),
    /// and then calls
    /// [`-[UIViewController setNeedsUpdateOfScreenEdgesDeferringSystemGestures]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/2887507-setneedsupdateofscreenedgesdefer?language=objc).
    ///
    /// This only has an effect on iOS 11.0+.
    fn set_preferred_screen_edges_deferring_system_gestures(&self, edges: ScreenEdge);

    /// Sets whether the [`Window`] prefers the status bar hidden.
    ///
    /// The default is to prefer showing the status bar.
    ///
    /// This changes the value returned by
    /// [`-[UIViewController prefersStatusBarHidden]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/1621440-prefersstatusbarhidden?language=objc),
    /// and then calls
    /// [`-[UIViewController setNeedsStatusBarAppearanceUpdate]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/1621354-setneedsstatusbarappearanceupdat?language=objc).
    fn set_prefers_status_bar_hidden(&self, hidden: bool);
}

impl WindowExtIOS for Window {
    #[inline]
    fn set_scale_factor(&self, scale_factor: f64) {
        self.window
            .maybe_queue_on_main(move |w| w.set_scale_factor(scale_factor))
    }

    #[inline]
    fn set_valid_orientations(&self, valid_orientations: ValidOrientations) {
        self.window
            .maybe_queue_on_main(move |w| w.set_valid_orientations(valid_orientations))
    }

    #[inline]
    fn set_prefers_home_indicator_hidden(&self, hidden: bool) {
        self.window
            .maybe_queue_on_main(move |w| w.set_prefers_home_indicator_hidden(hidden))
    }

    #[inline]
    fn set_preferred_screen_edges_deferring_system_gestures(&self, edges: ScreenEdge) {
        self.window.maybe_queue_on_main(move |w| {
            w.set_preferred_screen_edges_deferring_system_gestures(edges)
        })
    }

    #[inline]
    fn set_prefers_status_bar_hidden(&self, hidden: bool) {
        self.window
            .maybe_queue_on_main(move |w| w.set_prefers_status_bar_hidden(hidden))
    }
}

/// Additional methods on [`WindowBuilder`] that are specific to iOS.
pub trait WindowBuilderExtIOS {
    /// Sets the [`contentScaleFactor`] of the underlying [`UIWindow`] to `scale_factor`.
    ///
    /// The default value is device dependent, and it's recommended GLES or Metal applications set
    /// this to [`MonitorHandle::scale_factor()`].
    ///
    /// [`UIWindow`]: https://developer.apple.com/documentation/uikit/uiwindow?language=objc
    /// [`contentScaleFactor`]: https://developer.apple.com/documentation/uikit/uiview/1622657-contentscalefactor?language=objc
    fn with_scale_factor(self, scale_factor: f64) -> WindowBuilder;

    /// Sets the valid orientations for the [`Window`].
    ///
    /// The default value is [`ValidOrientations::LandscapeAndPortrait`].
    ///
    /// This sets the initial value returned by
    /// [`-[UIViewController supportedInterfaceOrientations]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/1621435-supportedinterfaceorientations?language=objc).
    fn with_valid_orientations(self, valid_orientations: ValidOrientations) -> WindowBuilder;

    /// Sets whether the [`Window`] prefers the home indicator hidden.
    ///
    /// The default is to prefer showing the home indicator.
    ///
    /// This sets the initial value returned by
    /// [`-[UIViewController prefersHomeIndicatorAutoHidden]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/2887510-prefershomeindicatorautohidden?language=objc).
    ///
    /// This only has an effect on iOS 11.0+.
    fn with_prefers_home_indicator_hidden(self, hidden: bool) -> WindowBuilder;

    /// Sets the screen edges for which the system gestures will take a lower priority than the
    /// application's touch handling.
    ///
    /// This sets the initial value returned by
    /// [`-[UIViewController preferredScreenEdgesDeferringSystemGestures]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/2887512-preferredscreenedgesdeferringsys?language=objc).
    ///
    /// This only has an effect on iOS 11.0+.
    fn with_preferred_screen_edges_deferring_system_gestures(
        self,
        edges: ScreenEdge,
    ) -> WindowBuilder;

    /// Sets whether the [`Window`] prefers the status bar hidden.
    ///
    /// The default is to prefer showing the status bar.
    ///
    /// This sets the initial value returned by
    /// [`-[UIViewController prefersStatusBarHidden]`](https://developer.apple.com/documentation/uikit/uiviewcontroller/1621440-prefersstatusbarhidden?language=objc).
    fn with_prefers_status_bar_hidden(self, hidden: bool) -> WindowBuilder;
}

impl WindowBuilderExtIOS for WindowBuilder {
    #[inline]
    fn with_scale_factor(mut self, scale_factor: f64) -> WindowBuilder {
        self.platform_specific.scale_factor = Some(scale_factor);
        self
    }

    #[inline]
    fn with_valid_orientations(mut self, valid_orientations: ValidOrientations) -> WindowBuilder {
        self.platform_specific.valid_orientations = valid_orientations;
        self
    }

    #[inline]
    fn with_prefers_home_indicator_hidden(mut self, hidden: bool) -> WindowBuilder {
        self.platform_specific.prefers_home_indicator_hidden = hidden;
        self
    }

    #[inline]
    fn with_preferred_screen_edges_deferring_system_gestures(
        mut self,
        edges: ScreenEdge,
    ) -> WindowBuilder {
        self.platform_specific
            .preferred_screen_edges_deferring_system_gestures = edges;
        self
    }

    #[inline]
    fn with_prefers_status_bar_hidden(mut self, hidden: bool) -> WindowBuilder {
        self.platform_specific.prefers_status_bar_hidden = hidden;
        self
    }
}

/// Additional methods on [`MonitorHandle`] that are specific to iOS.
pub trait MonitorHandleExtIOS {
    /// Returns a pointer to the [`UIScreen`] that is used by this monitor.
    ///
    /// [`UIScreen`]: https://developer.apple.com/documentation/uikit/uiscreen?language=objc
    fn ui_screen(&self) -> *mut c_void;

    /// Returns the preferred [`VideoMode`] for this monitor.
    ///
    /// This translates to a call to [`-[UIScreen preferredMode]`](https://developer.apple.com/documentation/uikit/uiscreen/1617823-preferredmode?language=objc).
    fn preferred_video_mode(&self) -> VideoMode;
}

impl MonitorHandleExtIOS for MonitorHandle {
    #[inline]
    fn ui_screen(&self) -> *mut c_void {
        // SAFETY: The marker is only used to get the pointer of the screen
        let mtm = unsafe { MainThreadMarker::new_unchecked() };
        Id::as_ptr(self.inner.ui_screen(mtm)) as *mut c_void
    }

    #[inline]
    fn preferred_video_mode(&self) -> VideoMode {
        VideoMode {
            video_mode: self.inner.preferred_video_mode(),
        }
    }
}

/// Valid orientations for a particular [`Window`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidOrientations {
    /// Excludes `PortraitUpsideDown` on iphone
    #[default]
    LandscapeAndPortrait,

    Landscape,

    /// Excludes `PortraitUpsideDown` on iphone
    Portrait,
}

/// The device [idiom].
///
/// [idiom]: https://developer.apple.com/documentation/uikit/uidevice/1620037-userinterfaceidiom?language=objc
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Idiom {
    Unspecified,

    /// iPhone and iPod touch.
    Phone,

    /// iPad.
    Pad,

    /// tvOS and Apple TV.
    TV,
    CarPlay,
}

bitflags! {
    /// The [edges] of a screen.
    ///
    /// [edges]: https://developer.apple.com/documentation/uikit/uirectedge?language=objc
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ScreenEdge: u8 {
        const NONE   = 0;
        const TOP    = 1 << 0;
        const LEFT   = 1 << 1;
        const BOTTOM = 1 << 2;
        const RIGHT  = 1 << 3;
        const ALL = ScreenEdge::TOP.bits() | ScreenEdge::LEFT.bits()
            | ScreenEdge::BOTTOM.bits() | ScreenEdge::RIGHT.bits();
    }
}
