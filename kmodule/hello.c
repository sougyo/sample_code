#include <linux/module.h>

MODULE_LICENSE("GPL");

static int __init my_module_init(void) {
	return 0;
}

static void __exit my_module_exit(void) {
}

module_init(my_module_init);
module_exit(my_module_exit);
