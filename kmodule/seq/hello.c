#include <linux/module.h>
#include <linux/seq_file.h>
#include <linux/proc_fs.h>

MODULE_LICENSE("GPL");

#define DATASIZE 10
int data[DATASIZE];

static void *my_seq_start(struct seq_file *s, loff_t *pos) {
	if (*pos == 0)
		return &data[0];
	else {
		*pos = 0;
		return NULL;
	}
}

static void *my_seq_next(struct seq_file *s, void *v, loff_t *pos) {
	int index = ++(*pos);
	if (index >= DATASIZE)
		return NULL;
	return &data[index];
}

static void my_seq_stop(struct seq_file *s, void *v) {
}

static int my_seq_show(struct seq_file *s, void *v) {
	seq_printf(s, "data: %d\n", *((int*)v));
	return 0;
}

static struct seq_operations my_seq_ops = {
	.start = my_seq_start,
	.next  = my_seq_next,
	.stop  = my_seq_stop,
	.show  = my_seq_show
};

static int my_open(struct inode *inode, struct file *file) {
	return seq_open(file, &my_seq_ops);
};

// see traverse() in fs/seq_file.c
static const struct proc_ops proc_example_operations = {
	.proc_open    = my_open,
	.proc_read    = seq_read,
	.proc_lseek   = seq_lseek,
	.proc_release = seq_release
};

static int __init my_module_init(void) {
	int i;
	for (i = 0; i < DATASIZE; i++)
		data[i] = i * i;

	proc_create("example", S_IRUGO | S_IWUGO, NULL, &proc_example_operations);
	return 0;
}

static void __exit my_module_exit(void) {
	remove_proc_entry("example", NULL);
}

module_init(my_module_init);
module_exit(my_module_exit);
