# The InitFs.ko module

Not much is required by the Kernel regarding the InitFs.ko module.
All it must do is to call `crate::modules::initfs::set_root_filesystem(device: Device)` as soon as it is done. The Device must contain at least `5` commands.

- Open
- Close
- Read
- Write
- IoCtl (GetInfo)

Open will be called immediately after the module returns. The IoCtl must be able to provide a number of information.

```
FS_GET_TOTAL_SIZE - Returns the size of the FileSystem in bytes
FS_GET_USED_SIZE - Returns, in bytes, how much of the FileSystem is used
```