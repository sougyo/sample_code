#include <linux/module.h>
#include <linux/seq_file.h>
#include <linux/proc_fs.h>

/*
kernel/params.c

#define STANDARD_PARAM_DEF(name, type, format, strtolfn)                \
        int param_set_##name(const char *val, const struct kernel_param *kp) \
        {                                                               \
                return strtolfn(val, 0, (type *)kp->arg);               \
        }                                                               \
        int param_get_##name(char *buffer, const struct kernel_param *kp) \
        {                                                               \
                return scnprintf(buffer, PAGE_SIZE, format "\n",        \
                                *((type *)kp->arg));                    \
        }                                                               \
        const struct kernel_param_ops param_ops_##name = {                      \
                .set = param_set_##name,                                \
                .get = param_get_##name,                                \
        };                                                              \
        EXPORT_SYMBOL(param_set_##name);                                \
        EXPORT_SYMBOL(param_get_##name);                                \
        EXPORT_SYMBOL(param_ops_##name)

STANDARD_PARAM_DEF(byte,        unsigned char,          "%hhu",         kstrtou8);
STANDARD_PARAM_DEF(short,       short,                  "%hi",          kstrtos16);
STANDARD_PARAM_DEF(ushort,      unsigned short,         "%hu",          kstrtou16);
STANDARD_PARAM_DEF(int,         int,                    "%i",           kstrtoint);
STANDARD_PARAM_DEF(uint,        unsigned int,           "%u",           kstrtouint);
STANDARD_PARAM_DEF(long,        long,                   "%li",          kstrtol);
STANDARD_PARAM_DEF(ulong,       unsigned long,          "%lu",          kstrtoul);
STANDARD_PARAM_DEF(ullong,      unsigned long long,     "%llu",         kstrtoull);
STANDARD_PARAM_DEF(hexint,      unsigned int,           "%#08x",        kstrtouint);
*/

MODULE_LICENSE("GPL");

int param1_val = 0;
module_param_cb(param1, &param_ops_int, &param1_val, 0600);

static int __init my_module_init(void) {
	return 0;
}

static void __exit my_module_exit(void) {
}

module_init(my_module_init);
module_exit(my_module_exit);

