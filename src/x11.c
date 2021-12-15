#include <sys/ipc.h>
#include <sys/shm.h>
#include <sys/types.h>

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/extensions/XShm.h>

#include <erl_nif.h>

static Display *display;
static Window window;
static GC context;
static XImage *image;
static XShmSegmentInfo *shm_info;

static ERL_NIF_TERM x11_window_create_nif(ErlNifEnv *env, int argc,
                                          const ERL_NIF_TERM argv[]) {
  uint16_t width, height;

  if (!enif_get_int(env, argv[0], &width)) {
    return enif_make_badarg(env);
  }

  if (!enif_get_int(env, argv[1], &height)) {
    return enif_make_badarg(env);
  }

  display = XOpenDisplay(NULL);
  if (display == NULL) {
    fprintf(stderr, "%s:%d: %s\n", __FILE__, __LINE__,
            "unable to connect to X server");
    exit(EXIT_FAILURE);
  }

  XVisualInfo visual_tpl = {
      .visualid =
          XVisualIDFromVisual(XDefaultVisual(display, XDefaultScreen(display))),
  };

  int visual_count = 0;

  XVisualInfo *visual_info = NULL;
  visual_info =
      XGetVisualInfo(display, VisualIDMask, &visual_tpl, &visual_count);

  Window window_root = 0;
  window_root = XRootWindow(display, visual_info->screen);

  int window_attrs_mask = 0;
  window_attrs_mask = CWEventMask | CWColormap | CWBorderPixel;

  XSetWindowAttributes window_attrs = {
      .event_mask = KeyPressMask | KeyReleaseMask | PointerMotionMask |
                    ButtonPressMask | ButtonReleaseMask,
      .colormap =
          XCreateColormap(display, window_root, visual_info->visual, AllocNone),
      .border_pixel = 0,
  };

  window = XCreateWindow(display, window_root, 0, 0, width, height, 0,
                         visual_info->depth, InputOutput, visual_info->visual,
                         window_attrs_mask, &window_attrs);

  XSizeHints window_hints = {
      .flags = PMinSize | PMaxSize,
      .min_width = width,
      .max_width = width,
      .min_height = height,
      .max_height = height,
  };
  XSetWMNormalHints(display, window, &window_hints);

  XStoreName(display, window, "Obscura");
  XMapWindow(display, window);

  shm_info = calloc(1, sizeof(XShmSegmentInfo));
  if (shm_info == NULL) {
    fprintf(stderr, "%s:%d: %s\n", __FILE__, __LINE__,
            "unable to allocate memory");
    exit(EXIT_FAILURE);
  }

  image = XShmCreateImage(display, visual_info->visual, visual_info->depth,
                          ZPixmap, NULL, shm_info, width, height);

  size_t image_size = image->bytes_per_line * image->height;
  shm_info->shmid = shmget((key_t)0, image_size, IPC_CREAT | 0777);

  shm_info->shmaddr = (char *)shmat(shm_info->shmid, 0, 0);
  image->data = shm_info->shmaddr;

  XShmAttach(display, shm_info);
  XSync(display, False);
  shmctl(shm_info->shmid, IPC_RMID, 0);

  XGCValues gc_values = {
      .graphics_exposures = False,
  };

  context = XCreateGC(display, window, GCGraphicsExposures, &gc_values);

  return enif_make_int(env, window);
}

static ERL_NIF_TERM x11_window_destroy_nif(ErlNifEnv *env, int argc,
                                           const ERL_NIF_TERM argv[]) {
  XShmDetach(display, shm_info);
  shmdt(shm_info->shmaddr);
  free(shm_info);

  XFree(image);

  XDestroyWindow(display, window);
  XCloseDisplay(display);

  return enif_make_int(env, 0);
}

static ERL_NIF_TERM x11_window_put_pixel_nif(ErlNifEnv *env, int argc,
                                             const ERL_NIF_TERM argv[]) {
  int x, y;
  uint32_t rgb;

  if (!enif_get_int(env, argv[0], &x)) {
    return enif_make_badarg(env);
  }

  if (!enif_get_int(env, argv[1], &y)) {
    return enif_make_badarg(env);
  }

  if (!enif_get_ulong(env, argv[2], &rgb)) {
    return enif_make_badarg(env);
  }

  int ret = -1;

  XWindowAttributes window_attrs;
  XGetWindowAttributes(display, window, &window_attrs);
  if (x < window_attrs.width && y < window_attrs.height) {
    ret = XPutPixel((XImage *)image, x, y, rgb);
  }

  return enif_make_int(env, ret);
}

static ERL_NIF_TERM x11_window_put_image_nif(ErlNifEnv *env, int argc,
                                             const ERL_NIF_TERM argv[]) {
  int ret;

  XWindowAttributes window_attrs;
  XGetWindowAttributes(display, window, &window_attrs);

  XPutImage(display, window, context, image, 0, 0, 0, 0, window_attrs.width,
            window_attrs.height);

  ret = XSync(display, False);

  return enif_make_int(env, ret);
}

static ErlNifFunc nif_funcs[] = {{"create", 2, x11_window_create_nif},
                                 {"destroy", 0, x11_window_destroy_nif},
                                 {"put_pixel", 3, x11_window_put_pixel_nif},
                                 {"put_image", 0, x11_window_put_image_nif}};

ERL_NIF_INIT(Elixir.Obscura.Window.X11, nif_funcs, NULL, NULL, NULL, NULL)
