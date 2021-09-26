#include <linux/module.h>
#include <linux/proc_fs.h>

MODULE_LICENSE("GPL");

int param_get_xxx(char *buffer, const struct kernel_param *kp) {
	printk(KERN_INFO "get called\n");
	return 0;
}

const struct kernel_param_ops param_ops_xxx = {
	.get = param_get_xxx,
};

int param1_val = 0;
module_param_cb(param1, &param_ops_xxx, &param1_val, 0600);

static int __init my_module_init(void) {
	return 0;
}

static void __exit my_module_exit(void) {
}

module_init(my_module_init);
module_exit(my_module_exit);

