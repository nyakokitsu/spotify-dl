use id3::{Tag, Version};
use id3::{frame::Picture, frame::PictureType};
use id3::TagLike;

pub struct MetadataTagger;

impl MetadataTagger {
    pub fn add_title_tag(mp3_data: &mut Vec<u8>, title: &str) -> anyhow::Result<()> {
        let mut tag = Tag::new();
        tag.set_title(title);
        let mut buf = Vec::new();
        tag.write_to(&mut buf, Version::Id3v24)?;
        // Prepend the tag to the mp3 data
        buf.append(mp3_data);
        *mp3_data = buf;
        Ok(())
    }

    pub fn add_author_tag(mp3_data: &mut Vec<u8>, author: &str) -> anyhow::Result<()> {
        let mut tag = Tag::new();
        tag.set_artist(author);
        let mut buf = Vec::new();
        tag.write_to(&mut buf, Version::Id3v24)?;
        // Prepend the tag to the mp3 data
        buf.append(mp3_data);
        *mp3_data = buf;
        Ok(())
    }

    pub fn add_cover_tag(mp3_data: &mut Vec<u8>, image_data: &[u8], mime_type: &str) -> anyhow::Result<()> {
        let mut tag = Tag::new();
        tag.add_frame(Picture {
            mime_type: mime_type.to_string(),
            picture_type: PictureType::CoverFront,
            description: "".to_string(),
            data: image_data.to_vec(),
        });
        let mut buf = Vec::new();
        tag.write_to(&mut buf, Version::Id3v24)?;
        // Prepend the tag to the mp3 data
        buf.append(mp3_data);
        *mp3_data = buf;
        Ok(())
    }
}