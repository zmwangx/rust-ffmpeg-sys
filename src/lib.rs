#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::approx_constant)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::ptr_offset_with_cast)]
#![allow(unpredictable_function_pointer_comparisons)]
#![allow(unnecessary_transmutes)]

extern crate libc;

#[cfg(feature = "vulkan")]
extern crate ash;

#[cfg(feature = "vulkan")]
use ash::vk::{
    AccessFlags as VkAccessFlagBits, Device as VkDevice, DeviceMemory as VkDeviceMemory,
    Format as VkFormat, Image as VkImage, ImageCreateFlags as VkImageCreateFlags,
    ImageLayout as VkImageLayout, ImageTiling as VkImageTiling,
    ImageUsageFlags as VkImageUsageFlagBits, Instance as VkInstance,
    MemoryPropertyFlags as VkMemoryPropertyFlagBits, PFN_vkGetInstanceProcAddr,
    PhysicalDevice as VkPhysicalDevice, QueueFlags as VkQueueFlagBits, Semaphore as VkSemaphore,
    VideoCodecOperationFlagsKHR as VkVideoCodecOperationFlagBitsKHR,
};

// the generated bindgen structs need these types that have lifetimes in them,
// but there is no way within bindgen to propagate those lifetimes out into the structs
// that contain these structs
//
// so, just put 'static to let it compile. Making sure the lifetimes are actually
// check out nicely is now part of the checks an author must do when using the unsafe
// functions that take in these structs or any other structs that contain them.

#[cfg(feature = "vulkan")]
type VkAllocationCallbacks = ash::vk::AllocationCallbacks<'static>;
#[cfg(feature = "vulkan")]
type VkPhysicalDeviceFeatures2 = ash::vk::PhysicalDeviceFeatures2<'static>;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
mod avutil;
pub use avutil::*;
