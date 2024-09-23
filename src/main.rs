use rfd::{MessageButtons, MessageDialog, MessageLevel};

fn init_vulkan() -> Result<(), Box<dyn std::error::Error>> {
    use ash::vk;
    unsafe {
        let entry = ash::Entry::load()?;
        let app_desc = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::default().application_info(&app_desc);
        let instance = entry.create_instance(&instance_desc, None)?;
        instance.destroy_instance(None);
    }
    Ok(())
}

fn main() {
    match init_vulkan() {
        Ok(_) => {
            MessageDialog::new()
                .set_level(MessageLevel::Info)
                .set_buttons(MessageButtons::Ok)
                .set_title("Success")
                .set_description("Load Vulkan Successfully")
                .show();
        }
        Err(error) => {
            MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_buttons(MessageButtons::Ok)
                .set_title("Error")
                .set_description(format!("Load Vulkan failed: {:?}", error))
                .show();
        }
    }
}
