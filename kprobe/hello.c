#include <linux/module.h>
#include <linux/kprobes.h>

MODULE_LICENSE("GPL");

static int handler_pre (struct kprobe *p, struct pt_regs *regs) {
        return 0;
}

static struct kprobe kp = {
        .symbol_name = "do_sys_open",
        .pre_handler = handler_pre,
};

static int __init kprobe_init(void) {
        int ret = register_kprobe(&kp);
        if (ret < 0) {
                pr_err("register_kprobe failed%d\n", ret);
                return ret;
        }
        return 0;
}

static void __exit kprobe_exit(void) {
        unregister_kprobe(&kp);
}

module_init(kprobe_init);
module_exit(kprobe_exit);
