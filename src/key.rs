use std::fmt;
const COMPACT_STRING_LENGTH_LENGTH: usize = 4;
// This might sound confusing but the ssh rfc mention that each string has its length inserted
// as a uint32 before its string bytes

pub struct Key {
    keyblob: Vec<u8>,
    comment: Vec<u8>,
    pub commentstr: String,
}

impl Key {
    pub fn from<I>(buf: &mut I) -> Self
    where
        I: Iterator<Item = u8>,
    {
        // It is expected that this is given the iterator pointed at the total keyblob size u32
        // It will go through the iterator and consume the bytes until its fully processed its key

        // The format is usually this
        // [total_key_blob_length][keytype_length][keytype_str][key_contents_length][key_contents][extra_stuff_length][extra_stuff_contents] + [comment_length][comment_str]
        // total_key_blob_length accounts for everything in the keyblob APPART from the comment as the special cert keys can include other
        // strings that we dont want, so we skip total_key_blob_length amount of the bytes until we
        // get to the comment which is not part of total_key_blob_length and easily extract it
        let keyblob_length: usize = u32::from_be_bytes(
            Key::extract_n_bytes(buf, COMPACT_STRING_LENGTH_LENGTH)
                .try_into()
                .unwrap(),
        ) as usize;

        let keyblob = Key::extract_n_bytes(buf, keyblob_length);

        let comment_length: usize = u32::from_be_bytes(
            Key::extract_n_bytes(buf, COMPACT_STRING_LENGTH_LENGTH)
                .try_into()
                .unwrap(),
        ) as usize;

        let comment: Vec<u8> = Key::extract_n_bytes(buf, comment_length);

        Key {
            keyblob,
            comment: comment.clone(),
            commentstr: Key::comment_to_string(&comment),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(&self.keyblob);
        bytes.extend(&self.comment);
        bytes
    }

    fn extract_n_bytes<I>(buf_iter: &mut I, num: usize) -> Vec<u8>
    where
        I: Iterator<Item = u8>,
    {
        buf_iter.take(num).collect()
    }

    fn comment_to_string(comment_bytes: &[u8]) -> String {
        String::from_utf8_lossy(comment_bytes).to_string()
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.commentstr)
    }
}
