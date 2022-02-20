#include <linux/module.h>
#include <linux/kthread.h>
#include <linux/delay.h>

MODULE_LICENSE("GPL");

static int kthread_func(void *arg) {
	while(!kthread_should_stop()){
		msleep(100);
		schedule();
	}
	return 0;
}

struct task_struct *task;
static int __init my_module_init(void) {
	int err;

	task = kthread_run(kthread_func, NULL, "kthread-test");
	if (IS_ERR(task)) {
		printk(KERN_INFO "kthread_run error\n");
		err = PTR_ERR(task);
		task = NULL;
		return err;
	}
	return 0;
}

static void __exit my_module_exit(void) {
	if (task)
		kthread_stop(task);
}

module_init(my_module_init);
module_exit(my_module_exit);
