#include <linux/fs.h>
#include <linux/cdev.h>
#include <linux/kern_levels.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include <linux/mutex.h>
#include <linux/uaccess.h>

DEFINE_MUTEX(parrot_mutex);

void mutex_lock_c(void) {
	mutex_lock(&parrot_mutex);
}

void mutex_unlock_c(void) {
	mutex_unlock(&parrot_mutex);
}

void init_counter_c(struct file *f) {
	f->private_data = 0;
}

size_t get_counter_c(struct file *f) {
	return (size_t)f->private_data;
}

void increment_counter_c(struct file *f) {
	f->private_data = (void *)(((size_t)f->private_data + 1) % 10);
}

struct file_operations fops;

void set_fops_c(
	int (*open)(struct inode *, struct file *),
	ssize_t (*read)(struct file *, char *, size_t, loff_t *),
	int (*release)(struct inode *, struct file *)
) {
	fops.owner = THIS_MODULE;
	fops.open = open;
	fops.read = read;
	fops.release = release;
}

dev_t device;

unsigned int get_chrdev_major_c(void) {
	return MAJOR(device);
}

unsigned int get_chrdev_minor_c(void) {
	return MINOR(device);
}

int alloc_chrdev_region_c(unsigned int first_minor, unsigned int count, char *name) {
	return alloc_chrdev_region(&device, first_minor, count, name);
}

void unregister_chrdev_region_c(unsigned int count) {
	unregister_chrdev_region(device, count);
}

struct cdev cdev;

void cdev_init_c(void) {
	cdev_init(&cdev, &fops);
}

int cdev_add_c(int count) {
	return cdev_add(&cdev, device, count);
}

void cdev_del_c(void) {
	cdev_del(&cdev);
}

extern unsigned long copy_to_user_c(void *to, const void *from, unsigned long count) {
    return copy_to_user(to, from, count);
};

MODULE_LICENSE("GPL v2");
