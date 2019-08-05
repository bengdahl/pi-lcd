#include "linux/fb.h"
#include <assert.h>
#include <inttypes.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>

typedef struct {
	size_t len;
	size_t xres;
	size_t yres;
	uint16_t* buf;
} fb;

fb get_fb() {
	int file = open("/dev/fb1", O_RDWR);
	assert(file > 0);
	struct fb_var_screeninfo info;
	assert(0 == ioctl(file, FBIOGET_VSCREENINFO, &info));
	size_t len = 2 * info.xres * info.yres;
	uint16_t *buf = mmap(NULL, len, PROT_READ | PROT_WRITE, MAP_SHARED, file, 0);
	assert(buf != MAP_FAILED);
	fb r = {len, info.xres, info.yres, buf};
	return r;
}
