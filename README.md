# driver_block

Common traits and types for block storage device drivers (i.e. disk).

## Examples

```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;
use alloc::vec;

use core::panic::PanicInfo;
use driver_common::{BaseDriverOps, DeviceType};
use driver_block::{ramdisk, BlockDriverOps};

const DISK_SIZE: usize = 0x1000;    // 4K
const BLOCK_SIZE: usize = 0x200;    // 512

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    axlog2::init("info");
    info!("[rt_ramdisk]: ...");

    axalloc::init();

    let mut disk = ramdisk::RamDisk::new(0x1000);
    assert_eq!(disk.device_type(), DeviceType::Block);
    assert_eq!(disk.device_name(), "ramdisk");
    assert_eq!(disk.block_size(), BLOCK_SIZE);
    assert_eq!(disk.num_blocks() as usize, DISK_SIZE/BLOCK_SIZE);

    let block_id = 1;

    let mut buf = vec![0u8; BLOCK_SIZE];
    assert!(disk.read_block(block_id, &mut buf).is_ok());
    assert!(buf[0..4] != *b"0123");

    buf[0] = b'0';
    buf[1] = b'1';
    buf[2] = b'2';
    buf[3] = b'3';

    assert!(disk.write_block(block_id, &buf).is_ok());
    assert!(disk.flush().is_ok());

    assert!(disk.read_block(block_id, &mut buf).is_ok());
    assert!(buf[0..4] == *b"0123");

    info!("[rt_ramdisk]: ok!");
    info!("[rt_driver_block]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}
```

## Re-exports

### `driver_common::BaseDriverOps`

```rust
pub trait BaseDriverOps: Send + Sync {
    // Required methods
    fn device_name(&self) -> &str;
    fn device_type(&self) -> DeviceType;
}
```

Common operations that require all device drivers to implement.

#### Required Methods

```rust
fn device_name(&self) -> &str
```

The name of the device.

```rust
fn device_type(&self) -> DeviceType
```

The type of the device.

#### Implementors

```rust
impl BaseDriverOps for AxDeviceEnum
impl BaseDriverOps for RamDisk
impl<H: Hal, T: Transport> BaseDriverOps for VirtIoBlkDev<H, T>
```

### `driver_common::DevError`

```rust
pub enum DevError {
    AlreadyExists,
    Again,
    BadState,
    InvalidParam,
    Io,
    NoMemory,
    ResourceBusy,
    Unsupported,
}
```

The error type for device operation failures.

#### Variants

**AlreadyExists**
An entity already exists.

**Again**
Try again, for non-blocking APIs.

**BadState**
Bad internal state.

**InvalidParam**
Invalid parameter/argument.

**Io**
Input/output error.

**NoMemory**
Not enough space/cannot allocate memory (DMA).

**ResourceBusy**
Device or resource is busy.

**Unsupported**
This operation is unsupported or unimplemented.

#### Trait Implementations

```rust
impl Debug for DevError
```

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. Read more

### `driver_common::DevResult`

```rust
pub type DevResult<T = ()> = Result<T, DevError>;\
```

A specialized Result type for device operations.

#### Aliased Type

```rust
enum DevResult<T = ()> {
    Ok(T),
    Err(DevError),
}
```

#### Variants

**Ok(T)**
Contains the success value

**Err(DevError)**
Contains the error value

#### Implementations

```rust
impl<T, E> Result<T, E>

impl<T, E> Result<&T, E>

impl<T, E> Result<&mut T, E>

impl<T, E> Result<Option<T>, E>

impl<T, E> Result<Result<T, E>, E>
```

### driver_common::DeviceType

```rust
pub enum DeviceType {
    Block,
    Char,
    Net,
    Display,
}
```

All supported device types.

#### Variants

**Block**
Block storage device (e.g., disk).

**Char**
Character device (e.g., serial port).

**Net**
Network device (e.g., ethernet card).

**Display**
Graphic display device (e.g., GPU)

#### Trait Implementations

##### `impl Clone for DeviceType`

```rust
fn clone(&self) -> DeviceType
```

Returns a copy of the value. Read more

```rust
fn clone_from(&mut self, source: &Self)
```

Performs copy-assignment from source. Read more

##### `impl Debug for DeviceType`

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> Result
```

Formats the value using the given formatter. Read more

##### `impl PartialEq for DeviceType`

```rust
fn eq(&self, other: &DeviceType) -> bool
```

This method tests for self and other values to be equal, and is used by ==.

```rust
fn ne(&self, other: &Rhs) -> bool
```

This method tests for !=. The default implementation is almost always sufficient, and should not be overridden without very good reason.

## Modules: ramdisk

Mock block devices that store data in RAM.

### Structs

#### `RamDisk`

```rust
pub struct RamDisk { /* private fields */ }
```

A RAM disk that stores data in a vector.

##### Implementations

**`impl RamDisk`**

```rust
pub fn new(size_hint: usize) -> Self
```

Creates a new RAM disk with the given size hint.

The actual size of the RAM disk will be aligned upwards to the block size (512 bytes).

```rust
pub fn from(buf: &[u8]) -> Self
Creates a new RAM disk from the exiting data.

The actual size of the RAM disk will be aligned upwards to the block size (512 bytes).

```rust
pub const fn size(&self) -> usize
```

Returns the size of the RAM disk in bytes.

##### Trait Implementations

**impl BaseDriverOps for RamDisk**

```rust
fn device_type(&self) -> DeviceType
```

The type of the device.

```rust
fn device_name(&self) -> &str
```

The name of the device.

```rust
impl BlockDriverOps for RamDisk

```rust
fn num_blocks(&self) -> u64
```

The number of blocks in this storage device. Read more

```rust
fn block_size(&self) -> usize
```

The size of each block in bytes.

```rust
fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> DevResult
```

Reads blocked data from the given block. Read more

```rust
fn write_block(&mut self, block_id: u64, buf: &[u8]) -> DevResult
```

Writes blocked data to the given block. Read more

```rust
fn flush(&mut self) -> DevResult
```

Flushes the device to write all pending data to the storage.

**`impl Default for RamDisk`**

```rust
fn default() -> RamDisk
```

Returns the “default value” for a type. Read more

## Traits

### `BlockDriverOps`

```rust
pub trait BlockDriverOps: BaseDriverOps {
    // Required methods
    fn num_blocks(&self) -> u64;
    fn block_size(&self) -> usize;
    fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> DevResult;
    fn write_block(&mut self, block_id: u64, buf: &[u8]) -> DevResult;
    fn flush(&mut self) -> DevResult;
}
```

Operations that require a block storage device driver to implement.

#### Required Methods

```rust
fn num_blocks(&self) -> u64
```

The number of blocks in this storage device.
The total size of the device is num_blocks() * block_size().

```rust
fn block_size(&self) -> usize
```

The size of each block in bytes.

```rust
fn read_block(&mut self, block_id: u64, buf: &mut [u8]) -> DevResult
```

Reads blocked data from the given block.
The size of the buffer may exceed the block size, in which case multiple contiguous blocks will be read.

```rust
fn write_block(&mut self, block_id: u64, buf: &[u8]) -> DevResult
```

Writes blocked data to the given block.
The size of the buffer may exceed the block size, in which case multiple contiguous blocks will be written.

```rust
fn flush(&mut self) -> DevResult
```

Flushes the device to write all pending data to the storage.

#### Implementors

```rust
impl BlockDriverOps for RamDisk
impl<H: Hal, T: Transport> BlockDriverOps for VirtIoBlkDev<H, T>
```
