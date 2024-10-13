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
    Device as VkDevice, Format as VkFormat, Image as VkImage,
    ImageCreateFlags as VkImageCreateFlags, ImageTiling as VkImageTiling,
    ImageUsageFlags as VkImageUsageFlagBits, Instance as VkInstance, PFN_vkGetInstanceProcAddr,
    PhysicalDevice as VkPhysicalDevice,
    DeviceMemory as VkDeviceMemory,
    MemoryPropertyFlags as VkMemoryPropertyFlagBits,
    AccessFlags as VkAccessFlagBits,
    ImageLayout as VkImageLayout,
    Semaphore as VkSemaphore,
    QueueFlags as VkQueueFlagBits,
    VideoCodecOperationFlagsKHR as VkVideoCodecOperationFlagBitsKHR
};

#[cfg(feature = "vulkan")]
type VkAllocationCallbacks = ash::vk::AllocationCallbacks<'static>; // hack!
#[cfg(feature = "vulkan")]
type VkPhysicalDeviceFeatures2 = ash::vk::PhysicalDeviceFeatures2<'static>; // hack!

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
mod avutil;
pub use avutil::*;
