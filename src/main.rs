use std::io::Read;

fn init_vulkan() -> Result<(), Box<dyn std::error::Error>> {
    use ash::vk;
    unsafe {
        let entry = ash::Entry::load()?;
        let app_desc = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::default().application_info(&app_desc);
        let instance = entry.create_instance(&instance_desc, None)?;
        for (index, device) in instance.enumerate_physical_devices()?.iter().enumerate() {
            let properties = instance.get_physical_device_properties(*device);
            let name = properties.device_name;
            let name = String::from_utf8(name.map(|i| i as u8).to_vec())?;
            let device_type = properties.device_type;
            println!(
                "Device: {}\nName: {}\nType: {:?}\n",
                index, name, device_type
            );
        }
        instance.destroy_instance(None);
    }
    Ok(())
}

fn main() {
    match init_vulkan() {
        Ok(_) => {
            println!("✅ Load Vulkan Successfully");
        }
        Err(error) => {
            println!("❌ Load Vulkan failed: {:?}", error);
        }
    }
    let _ = std::io::stdin().read(&mut [0u8]).unwrap();
}
