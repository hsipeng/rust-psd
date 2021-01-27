use crate::sections::MajorSections;
use crate::sections::file_header_section::{FileHeaderSection};
use failure::{Error};

/// Represents the contents of a PSD file
///
/// ## PSB Support
///
/// We do not currently support PSB since the original authors didn't need it, but adding
/// support should be trivial. If you'd like to support PSB please open an issue.
#[derive(Debug)]
pub struct Psd {
    file_header_section: FileHeaderSection,
    // image_resources_section: ImageResourcesSection,
    // layer_and_mask_information_section: LayerAndMaskInformationSection,
    // image_data_section: ImageDataSection,
}

impl Psd {
    /// Create a Psd from a byte slice.
    ///
    /// You'll typically get these bytes from a PSD file.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let psd_bytes = include_bytes!("./my-psd-file.psd");
    ///
    /// let psd = Psd::from_bytes(psd_bytes);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Result<Psd, Error> {
        let major_sections = MajorSections::from_bytes(bytes)?;

        let file_header_section = FileHeaderSection::from_bytes(major_sections.file_header)?;

        // let psd_width = file_header_section.width.0;
        // let psd_height = file_header_section.height.0;
        // let channel_count = file_header_section.channel_count.count();

        Ok(Psd {
            file_header_section,
            // image_resources_section,
            // layer_and_mask_information_section,
            // image_data_section,
        })
    }
}

// Methods for working with the file section header
impl Psd {
    /// The width of the PSD file
    pub fn width(&self) -> u32 {
        self.file_header_section.width.0
    }

    /// The height of the PSD file
    pub fn height(&self) -> u32 {
        self.file_header_section.height.0
    }
}