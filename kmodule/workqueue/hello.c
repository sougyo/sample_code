#include <linux/module.h>
#include <linux/workqueue.h>
#include <linux/slab.h>
#include <linux/delay.h>

MODULE_LICENSE("GPL");

static struct workqueue_struct *wq;
struct work_struct *work;

static void work_handler(struct work_struct *work) {
	static int i = 0;
	printk(KERN_INFO "work_handler called %d\n", i);
	if (i++ < 10)
		queue_work(wq, work);
}

static int __init my_module_init(void) {
	wq = create_workqueue("my_work_queue");
	if (!wq)
		return -ENOMEM;

	work = (struct work_struct*)kmalloc(sizeof(struct work_struct), GFP_KERNEL);
	if (!work) {
		destroy_workqueue(wq);
		wq = NULL;
		return -ENOMEM;
	}
	INIT_WORK(work, work_handler);

	queue_work(wq, work);

	return 0;
}

static void __exit my_module_exit(void) {
	if (wq)
		destroy_workqueue(wq);

	if (work)
		kfree(work);
}

module_init(my_module_init);
module_exit(my_module_exit);
