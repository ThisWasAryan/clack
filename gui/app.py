"""
Main GTK Application class and dependency checking.

Provides the `ClackApp` class which manages the application lifecycle
and ensures all required system dependencies are met before starting.
"""

import shutil
import subprocess
import sys
import gi
gi.require_version('Gtk', '4.0')
from gi.repository import Gtk, Gio

def show_error_dialog(message: str, is_fatal: bool = True):
    from gi.repository import Gtk, GLib
    
    app = Gtk.Application(application_id="com.example.clack.error")
    
    def on_activate(app):
        dialog = Gtk.MessageDialog(
            transient_for=None,
            message_type=Gtk.MessageType.ERROR,
            buttons=Gtk.ButtonsType.OK,
            text="Dependency Error",
            secondary_text=message,
        )
        
        def on_response(dialog, response_id):
            dialog.close()
            if is_fatal:
                app.quit()
        
        dialog.connect("response", on_response)
        dialog.set_application(app)
        dialog.show()
        
    app.connect("activate", on_activate)
    app.run(None)

def check_dependencies() -> bool:
    """
    Checks for required and optional system dependencies.
    """
    if not shutil.which('clack'):
        show_error_dialog("Missing 'clack' binary. Please ensure clack is compiled and in your PATH.")
        return False
        
    if not shutil.which('ydotool'):
        show_error_dialog("Missing 'ydotool' binary. Please install ydotool.")
        return False
        
    if shutil.which('pgrep'):
        result = subprocess.run(['pgrep', 'ydotoold'], capture_output=True)
        if result.returncode != 0:
            show_error_dialog("ydotoold is not running. Please start it with 'sudo ydotoold --socket-perm 0666 &'")
            return False
        
    if not shutil.which('xdotool'):
        print("WARNING: Missing 'xdotool' binary. Focus detection will be disabled.", file=sys.stderr)
        
    return True

class ClackApp(Gtk.Application):
    """
    The main GTK Application subclass for Clack GUI.
    """
    def __init__(self):
        super().__init__(application_id="com.example.clack", flags=Gio.ApplicationFlags.NON_UNIQUE)
        self.window = None

    def do_activate(self):
        from .window import ClackWindow
        from .state import ClackState
        from .clack_runner import ClackRunner
        
        if not self.window:
            state = ClackState()
            from .persistence import load_settings
            load_settings(state)
            runner = ClackRunner()
            self.window = ClackWindow(application=self, state=state, runner=runner)
        self.window.present()
