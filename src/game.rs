//! Game loop.

// Extern crate.
use std::mem::replace;

// Local crate.
use GameWindow = game_window::GameWindow;
use GameIteratorSettings;
use GameIterator;
use KeyPress;
use KeyPressArgs;
use KeyRelease;
use KeyReleaseArgs;
use MouseMove;
use MouseMoveArgs;
use MouseRelativeMove;
use MouseRelativeMoveArgs;
use MousePress;
use MousePressArgs;
use MouseRelease;
use MouseReleaseArgs;
use Render;
use RenderArgs;
use Update;
use UpdateArgs;

/// Implemented by game applications.
pub trait Game<R>: Copy {
    /// Render graphics.
    fn render(&mut self, _resouces: &mut R, _args: &mut RenderArgs) {}

    /// Update the physical state of the game.
    fn update(&mut self, _args: &mut UpdateArgs) {}

    /// Perform tasks for loading before showing anything.
    fn load(&mut self) {}

    /// User pressed a key.
    ///
    /// This can be overridden to handle key pressed events.
    fn key_press(&mut self, _args: &KeyPressArgs) {}

    /// User released a key.
    ///
    /// This can be overridden to handle key released events.
    fn key_release(&mut self, _args: &KeyReleaseArgs) {}

    /// Pressed a mouse button.
    fn mouse_press(&mut self, _args: &MousePressArgs) {}

    /// Released a mouse button.
    fn mouse_release(&mut self, _args: &MouseReleaseArgs) {}

    /// Moved mouse cursor.
    fn mouse_move(&mut self, _args: &MouseMoveArgs) {}

    /// Moved mouse relative, not bounded by cursor.
    fn mouse_relative_move(&mut self, _args: &MouseRelativeMoveArgs) {}

    /// Executes a game loop.
    fn run<W: GameWindow>(
        mut self,
        game_window: &mut W,
        game_iter_settings: &GameIteratorSettings,
        render_resources: &mut R
    ) {
        let mut game_iter = GameIterator::new(
            game_window,
            game_iter_settings
        );

        self.load();

        let mut buf2 = self;

        loop {
            match game_iter.next() {
                None => break,
                Some(mut e) => match e {
                    Render(ref mut args) => {    
                        replace( &mut buf2, self );
                        buf2.render(render_resources, args);
                    },
                    Update(ref mut args) => self.update(args),
                    KeyPress(ref args) => self.key_press(args),
                    KeyRelease(ref args) => self.key_release(args),
                    MousePress(ref args) => self.mouse_press(args),
                    MouseRelease(ref args) => self.mouse_release(args),
                    MouseMove(ref args) => self.mouse_move(args),
                    MouseRelativeMove(ref args) => self.mouse_relative_move(args),
                }
            }
        }
    }
}

