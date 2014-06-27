//! Game loop.

//extern crate debug;

// Extern crate.
use std::mem::replace;

// Local crate.
use game_window::{
    GraphicsWindow,
    GameWindow,
};
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
pub trait Game<R: Send>: Copy + Send {
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
    fn run<EW: GameWindow, GW: GraphicsWindow> (
        mut self,
        event_window: EW,
        graphics_window: GW,
        game_iter_settings: GameIteratorSettings,
        mut render_resources: R
    ) {

        self.load();

//        let mut buf2 = self;
        let (tx, rx) = sync_channel(0);

        // Everything but render thread
        spawn(proc() {
            let mut buf2 = self;
            let mut game_window = event_window;

            let mut game_iter = GameIterator::new(
                &mut game_window,
                &game_iter_settings
            );

            loop {
                match game_iter.next() {
                    None => break,
                    Some(mut e) => match e {
                        Render(args) => {    
                            
                            //let (n, no): (int, int) = (self, args);
                            tx.send((buf2, args));
                            //replace( &mut buf2, self );
                            //buf2.render(render_resources, args);
                        },
                        Update(ref mut args) => buf2.update(args),
                        KeyPress(ref args) => buf2.key_press(args),
                        KeyRelease(ref args) => buf2.key_release(args),
                        MousePress(ref args) => buf2.mouse_press(args),
                        MouseRelease(ref args) => buf2.mouse_release(args),
                        MouseMove(ref args) => buf2.mouse_move(args),
                        MouseRelativeMove(ref args) => buf2.mouse_relative_move(args),
                    }
                }
            }
        });

        // Render Thread

        loop {
            let (mut buf2, mut args): (Self, RenderArgs) = rx.recv();
            // println!("{:?}\n", buf2);
            buf2.render( &mut render_resources, &mut args);
            graphics_window.swap_buffers();
        }
    }
}

