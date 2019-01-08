/* example.c
* example ARM Linux bootloader code
* this example is distributed under the BSD licence
*/

/* list of possible tags */
#define ATAG_NONE       0x00000000
#define ATAG_CORE       0x54410001
#define ATAG_MEM        0x54410002
#define ATAG_VIDEOTEXT  0x54410003
#define ATAG_RAMDISK    0x54410004
#define ATAG_INITRD2    0x54420005
#define ATAG_SERIAL     0x54410006
#define ATAG_REVISION   0x54410007
#define ATAG_VIDEOLFB   0x54410008
#define ATAG_CMDLINE    0x54410009

/* structures for each atag */
struct atag_header {
	u32 size; /* length of tag in words including this header */
	u32 tag;  /* tag type */
};

struct atag_core {
	u32 flags;
	u32 pagesize;
	u32 rootdev;
};

struct atag_mem {
	u32     size;
	u32     start;
};

struct atag_videotext {
	u8              x;
	u8              y;
	u16             video_page;
	u8              video_mode;
	u8              video_cols;
	u16             video_ega_bx;
	u8              video_lines;
	u8              video_isvga;
	u16             video_points;
};

struct atag_ramdisk {
	u32 flags;
	u32 size;
	u32 start;
};

struct atag_initrd2 {
	u32 start;
	u32 size;
};

struct atag_serialnr {
	u32 low;
	u32 high;
};

struct atag_revision {
	u32 rev;
};

struct atag_videolfb {
	u16             lfb_width;
	u16             lfb_height;
	u16             lfb_depth;
	u16             lfb_linelength;
	u32             lfb_base;
	u32             lfb_size;
	u8              red_size;
	u8              red_pos;
	u8              green_size;
	u8              green_pos;
	u8              blue_size;
	u8              blue_pos;
	u8              rsvd_size;
	u8              rsvd_pos;
};

struct atag_cmdline {
	char    cmdline[1];
};

struct atag {
	struct atag_header hdr;
	union {
		struct atag_core         core;
		struct atag_mem          mem;
		struct atag_videotext    videotext;
		struct atag_ramdisk      ramdisk;
		struct atag_initrd2      initrd2;
		struct atag_serialnr     serialnr;
		struct atag_revision     revision;
		struct atag_videolfb     videolfb;
		struct atag_cmdline      cmdline;
	} u;
};


#define tag_next(t)     ((struct tag *)((u32 *)(t) + (t)->hdr.size))
#define tag_size(type)  ((sizeof(struct tag_header) + sizeof(struct type)) >> 2)
static struct atag *params; /* used to point at the current tag */

static void
setup_core_tag(void * address, long pagesize)
{
	params = (struct tag *)address;         /* Initialise parameters to start at given address */

	params->hdr.tag = ATAG_CORE;            /* start with the core tag */
	params->hdr.size = tag_size(atag_core); /* size the tag */

	params->u.core.flags = 1;               /* ensure read-only */
	params->u.core.pagesize = pagesize;     /* systems pagesize (4k) */
	params->u.core.rootdev = 0;             /* zero root device (typicaly overidden from commandline )*/

	params = tag_next(params);              /* move pointer to next tag */
}

static void
setup_ramdisk_tag(u32_t size)
{
	params->hdr.tag = ATAG_RAMDISK;         /* Ramdisk tag */
	params->hdr.size = tag_size(atag_ramdisk);  /* size tag */

	params->u.ramdisk.flags = 0;            /* Load the ramdisk */
	params->u.ramdisk.size = size;          /* Decompressed ramdisk size */
	params->u.ramdisk.start = 0;            /* Unused */

	params = tag_next(params);              /* move pointer to next tag */
}

static void
setup_initrd2_tag(u32_t start, u32_t size)
{
	params->hdr.tag = ATAG_INITRD2;         /* Initrd2 tag */
	params->hdr.size = tag_size(atag_initrd2);  /* size tag */

	params->u.initrd2.start = start;        /* physical start */
	params->u.initrd2.size = size;          /* compressed ramdisk size */

	params = tag_next(params);              /* move pointer to next tag */
}

static void
setup_mem_tag(u32_t start, u32_t len)
{
	params->hdr.tag = ATAG_MEM;             /* Memory tag */
	params->hdr.size = tag_size(atag_mem);  /* size tag */

	params->u.mem.start = start;            /* Start of memory area (physical address) */
	params->u.mem.size = len;               /* Length of area */

	params = tag_next(params);              /* move pointer to next tag */
}

static void
setup_cmdline_tag(const char * line)
{
	int linelen = strlen(line);

	if (!linelen)
		return;                             /* do not insert a tag for an empty commandline */

	params->hdr.tag = ATAG_CMDLINE;         /* Commandline tag */
	params->hdr.size = (sizeof(struct atag_header) + linelen + 1 + 4) >> 2;

	strcpy(params->u.cmdline.cmdline, line); /* place commandline into tag */

	params = tag_next(params);              /* move pointer to next tag */
}

static void
setup_end_tag(void)
{
	params->hdr.tag = ATAG_NONE;            /* Empty tag ends list */
	params->hdr.size = 0;                   /* zero length */
}


#define DRAM_BASE 0x10000000
#define ZIMAGE_LOAD_ADDRESS DRAM_BASE + 0x8000
#define INITRD_LOAD_ADDRESS DRAM_BASE + 0x800000

static void
setup_tags(parameters)
{
	setup_core_tag(parameters, 4096);       /* standard core tag 4k pagesize */
	setup_mem_tag(DRAM_BASE, 0x4000000);    /* 64Mb at 0x10000000 */
	setup_mem_tag(DRAM_BASE + 0x8000000, 0x4000000); /* 64Mb at 0x18000000 */
	setup_ramdisk_tag(4096);                /* create 4Mb ramdisk */
	setup_initrd2_tag(INITRD_LOAD_ADDRESS, 0x100000); /* 1Mb of compressed data placed 8Mb into memory */
	setup_cmdline_tag("root=/dev/ram0");    /* commandline setting root device */
	setup_end_tag(void);                    /* end of tags */
}

int
start_linux(char *name, char *rdname)
{
	void(*theKernel)(int zero, int arch, u32 params);
	u32 exec_at = (u32)-1;
	u32 parm_at = (u32)-1;
	u32 machine_type;

	exec_at = ZIMAGE_LOAD_ADDRESS;
	parm_at = DRAM_BASE + 0x100;

	load_image(name, exec_at);              /* copy image into RAM */

	load_image(rdname, INITRD_LOAD_ADDRESS);/* copy initial ramdisk image into RAM */

	setup_tags(parm_at);                    /* sets up parameters */

	machine_type = get_mach_type();         /* get machine type */

	irq_shutdown();                         /* stop irq */

	cpu_op(CPUOP_MMUCHANGE, NULL);          /* turn MMU off */

	theKernel = (void(*)(int, int, u32))exec_at; /* set the kernel address */

	theKernel(0, machine_type, parm_at);    /* jump to kernel with register set */

	return 0;
}