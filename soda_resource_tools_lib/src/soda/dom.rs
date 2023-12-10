use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    str::FromStr,
};

use serde_json::Value;
use xml::{writer::XmlEvent, EventWriter};

pub(crate) fn write_text_element(w: &mut EventWriter<&mut Vec<u8>>, element: &str, value: &str) {
    w.write(XmlEvent::start_element(element)).unwrap();
    w.write(XmlEvent::characters(value)).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
}

pub(crate) fn write_cdata_text_element(w: &mut EventWriter<&mut Vec<u8>>, element: &str, value: &str) {
    w.write(XmlEvent::start_element(element)).unwrap();
    w.write(XmlEvent::cdata(value)).unwrap();
    w.write(XmlEvent::end_element()).unwrap();
}

pub(crate) fn save_nfo(xml: &Vec<u8>, file_path: &str) {
    // if file exist, delete it
    if Path::new(file_path).exists() {
        fs::remove_file(file_path).unwrap();
    }

    // gen xml
    let str = std::str::from_utf8(xml).unwrap();

    // write str to file
    let mut file = File::create(file_path).unwrap();
    file.write_all(str.as_bytes()).unwrap();
}

#[cfg(test)]
mod dom_tests {
    use std::{fs::File, io::Write};
}
