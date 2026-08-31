#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <X11/Xlib.h>
#include <X11/Xatom.h>
#include <X11/extensions/XTest.h>

int minecraft_is_active(Display *display) {
    Window root = DefaultRootWindow(display);
    Atom active_atom = XInternAtom(display, "_NET_ACTIVE_WINDOW", False);

    Atom actual_type;
    int actual_format;
    unsigned long nitems, bytes_after;
    unsigned char *data = NULL;

    if (XGetWindowProperty(
        display,
        root,
        active_atom,
        0,
        1,
        False,
        XA_WINDOW,
        &actual_type,
        &actual_format,
        &nitems,
        &bytes_after,
        &data
    ) != Success || !data) {
        return 0;
    }

    Window active_window = *(Window *)data;
    XFree(data);

    if (!active_window)
        return 0;

    Atom utf8 = XInternAtom(display, "UTF8_STRING", False);
    Atom net_wm_name = XInternAtom(display, "_NET_WM_NAME", False);

    data = NULL;

    if (XGetWindowProperty(
        display,
        active_window,
        net_wm_name,
        0,
        1024,
        False,
        utf8,
        &actual_type,
        &actual_format,
        &nitems,
        &bytes_after,
        &data
    ) != Success || !data) {
        return 0;
    }

    int result = strstr((char *)data, "Minecraft") != NULL;

    XFree(data);

    return result;
}

int main(void) {
    Display *display = XOpenDisplay(NULL);

    if (!display) {
        fprintf(stderr, "Failed to open X display\n");
        return 1;
    }

    printf("Auto-clicker running.\n");
    printf("Only clicks when Minecraft is active.\n");
    printf("Ctrl+C to stop.\n");

    while (1) {
        if (minecraft_is_active(display)) {
            XTestFakeButtonEvent(display, 1, True, CurrentTime);
            XTestFakeButtonEvent(display, 1, False, CurrentTime);
            XFlush(display);

            printf("Clicked!\n");
        }

        sleep(10);
    }

    XCloseDisplay(display);
    return 0;
}
