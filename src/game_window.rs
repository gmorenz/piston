//! A trait for window operations.

use event;
use game_window_settings::GameWindowSettings;

/// Implemented by window back-end.
pub trait GraphicsWindow  {
    /// If window support double buffers, called this to tell implementation
    /// swap buffers.
    fn swap_buffers(&self) {}
}

/// Implemented by window back-end.
pub trait GameWindow: Send {
    /// Get the window's settings.
    fn get_settings<'a>(&'a self) -> &'a GameWindowSettings;

    /// Returns ture if the window should close.
    fn should_close(&self) -> bool;

    /// Get the window's size
    fn get_size(&self) -> (u32, u32) {
        (self.get_settings().size[0], self.get_settings().size[1])
    }

    /// Poll a event from window's event queue.
    fn poll_event(&mut self) -> event::Event { event::NoEvent }

}
