#include <linux/module.h>
#include <linux/workqueue.h>
#include <linux/slab.h>
#include <linux/delay.h>

MODULE_LICENSE("GPL");

static struct workqueue_struct *wq;
struct delayed_work *work;

static void work_handler(struct work_struct *w) {
	static int i = 0;
	printk(KERN_INFO "work_handler called %d\n", i);
	if (i++ < 10)
		queue_delayed_work(wq, work, 1000);
}

static int __init my_module_init(void) {
	wq = create_workqueue("my_work_queue");
	if (!wq)
		return -ENOMEM;

	work = (struct delayed_work*)kmalloc(sizeof(struct delayed_work), GFP_KERNEL);
	if (!work) {
		destroy_workqueue(wq);
		wq = NULL;
		return -ENOMEM;
	}
	INIT_DELAYED_WORK(work, work_handler);

	queue_delayed_work(wq, work, 1000);

	return 0;
}

static void __exit my_module_exit(void) {
	if (work)
		cancel_delayed_work(work);

	if (wq)
		destroy_workqueue(wq);

	if (work)
		kfree(work);
}

module_init(my_module_init);
module_exit(my_module_exit);
