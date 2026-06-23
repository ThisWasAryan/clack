"""
Entry point for the Clack GUI application.

This module is executed when running `python -m gui`.
It performs initial environment checks and launches the GTK application.
"""

import sys

def main():
    try:
        import gi
        gi.require_version('Gtk', '4.0')
        from gi.repository import Gtk, GLib
        print("GTK4 import OK")
    except Exception as e:
        print(f"Failed to load GTK4: {e}")
        print("Please ensure Python bindings for GTK4 are installed.")
        sys.exit(1)

    from gui.app import check_dependencies, ClackApp
    if not check_dependencies():
        sys.exit(1)
        
    app = ClackApp()
    sys.exit(app.run(sys.argv))

if __name__ == '__main__':
    sys.exit(main())
