/*!
What's this all about then?
*/

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

use crate::common::xml::write::*;
use crate::description::TypeID;
use crate::error::{xml_error, Error};
use crate::syntax::{
    XML_ELEM_DEVICE, XML_ELEM_DEVICE_LIST, XML_ELEM_DEVICE_TYPE, XML_ELEM_FRIENDLY_NAME,
    XML_ELEM_ICON, XML_ELEM_ICON_DEPTH, XML_ELEM_ICON_HEIGHT, XML_ELEM_ICON_LIST,
    XML_ELEM_ICON_MIME_TYPE, XML_ELEM_ICON_URL, XML_ELEM_ICON_WIDTH, XML_ELEM_MANUFACTURER,
    XML_ELEM_MANUFACTURER_URL, XML_ELEM_MODEL_DESCR, XML_ELEM_MODEL_NAME, XML_ELEM_MODEL_NUMBER,
    XML_ELEM_MODEL_URL, XML_ELEM_PRESENTATION_URL, XML_ELEM_ROOT, XML_ELEM_SERIAL_NUMBER,
    XML_ELEM_SERVICE, XML_ELEM_SERVICE_CONTROL_URL, XML_ELEM_SERVICE_EVENT_URL,
    XML_ELEM_SERVICE_ID, XML_ELEM_SERVICE_LIST, XML_ELEM_SERVICE_SCPD_URL, XML_ELEM_SERVICE_TYPE,
    XML_ELEM_UDN, XML_ELEM_UPC, XML_ELEM_URL_BASE, XML_NS_DEVICE, XML_NS_SERVICE,
};
use crate::SpecVersion;
use quick_xml::Writer;
use std::io::Write;

#[derive(Clone, Debug)]
pub struct Icon {
    pub mime_type: String,
    pub width: u16,
    pub height: u16,
    pub depth: u16,
    pub url: String, /* URL */
}

#[derive(Clone, Debug)]
pub struct Service {
    pub service_type: TypeID,
    pub service_id: String,    /* URI */
    pub scpd_url: String,      /* URL */
    pub control_url: String,   /* URL */
    pub event_sub_url: String, /* URL */
}

#[derive(Clone, Debug)]
pub struct Device {
    pub device_type: TypeID,
    pub friendly_name: String,
    pub manufacturer: String,
    pub manufacturer_url: Option<String>, /* URL */
    pub model_description: Option<String>,
    pub model_name: String,
    pub model_number: Option<String>,
    pub model_url: Option<String>, /* URL */
    pub serial_number: Option<String>,
    pub unique_device_name: String,
    pub upc: Option<String>,
    pub icon_list: Vec<Icon>,
    pub service_list: Vec<Service>,
    pub device_list: Vec<Device>,
    pub presentation_url: Option<String>, /* URL */
}

#[derive(Clone, Debug)]
pub struct DeviceRoot {
    pub spec_version: SpecVersion,
    pub url_base: String, /* URL */
    pub device: Device,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn to_writer<T: Write>(root: &DeviceRoot, writer: T) -> Result<T, Error> {
    root.write_root(writer)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T: Write> RootWritable<T> for DeviceRoot {}

impl<T: Write> Writable<T> for DeviceRoot {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let root =
            start_ns_element(writer, XML_ELEM_ROOT, XML_NS_DEVICE, None).map_err(xml_error)?;

        let _ = self.spec_version.write(writer)?;

        text_element(
            writer,
            XML_ELEM_URL_BASE,
            self.url_base.to_string().as_bytes(),
        )
        .map_err(xml_error)?;

        self.device.write(writer)?;

        root.end(writer).map_err(xml_error)
    }
}

impl<T: Write> Writable<T> for Device {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let top = start_element(writer, XML_ELEM_DEVICE).map_err(xml_error)?;

        text_element(
            writer,
            XML_ELEM_DEVICE_TYPE,
            self.device_type.to_string().as_bytes(),
        )
        .map_err(xml_error)?;

        text_element(
            writer,
            XML_ELEM_FRIENDLY_NAME,
            self.friendly_name.as_bytes(),
        )
        .map_err(xml_error)?;

        text_element(writer, XML_ELEM_MANUFACTURER, self.manufacturer.as_bytes())
            .map_err(xml_error)?;

        if let Some(s) = &self.manufacturer_url {
            text_element(writer, XML_ELEM_MANUFACTURER_URL, s.as_bytes()).map_err(xml_error)?;
        }

        if let Some(s) = &self.model_description {
            text_element(writer, XML_ELEM_MODEL_DESCR, s.as_bytes()).map_err(xml_error)?;
        }

        text_element(writer, XML_ELEM_MODEL_NAME, self.model_name.as_bytes()).map_err(xml_error)?;

        if let Some(s) = &self.model_number {
            text_element(writer, XML_ELEM_MODEL_NUMBER, s.as_bytes()).map_err(xml_error)?;
        }

        if let Some(s) = &self.model_url {
            text_element(writer, XML_ELEM_MODEL_URL, s.as_bytes()).map_err(xml_error)?;
        }

        if let Some(s) = &self.serial_number {
            text_element(writer, XML_ELEM_SERIAL_NUMBER, s.as_bytes()).map_err(xml_error)?;
        }

        text_element(writer, XML_ELEM_UDN, self.unique_device_name.as_bytes())
            .map_err(xml_error)?;

        if let Some(s) = &self.upc {
            text_element(writer, XML_ELEM_UPC, s.as_bytes()).map_err(xml_error)?;
        }

        if !&self.icon_list.is_empty() {
            let list = start_element(writer, XML_ELEM_ICON_LIST).map_err(xml_error)?;
            for icon in &self.icon_list {
                icon.write(writer)?;
            }
            list.end(writer).map_err(xml_error)?;
        }

        if !&self.service_list.is_empty() {
            let list = start_element(writer, XML_ELEM_SERVICE_LIST).map_err(xml_error)?;
            for service in &self.service_list {
                service.write(writer)?;
            }
            list.end(writer).map_err(xml_error)?;
        }

        if !&self.device_list.is_empty() {
            let list = start_element(writer, XML_ELEM_DEVICE_LIST).map_err(xml_error)?;
            for device in &self.device_list {
                device.write(writer)?;
            }
            list.end(writer).map_err(xml_error)?;
        }

        if let Some(s) = &self.presentation_url {
            text_element(writer, XML_ELEM_PRESENTATION_URL, s.as_bytes()).map_err(xml_error)?;
        }

        top.end(writer).map_err(xml_error)
    }
}

impl<T: Write> Writable<T> for Icon {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let element = start_element(writer, XML_ELEM_ICON).map_err(xml_error)?;

        text_element(writer, XML_ELEM_ICON_MIME_TYPE, self.mime_type.as_bytes())
            .map_err(xml_error)?;
        text_element(
            writer,
            XML_ELEM_ICON_WIDTH,
            self.width.to_string().as_bytes(),
        )
        .map_err(xml_error)?;
        text_element(
            writer,
            XML_ELEM_ICON_HEIGHT,
            self.height.to_string().as_bytes(),
        )
        .map_err(xml_error)?;
        text_element(
            writer,
            XML_ELEM_ICON_DEPTH,
            self.depth.to_string().as_bytes(),
        )
        .map_err(xml_error)?;
        text_element(writer, XML_ELEM_ICON_URL, self.url.as_bytes()).map_err(xml_error)?;

        element.end(writer).map_err(xml_error)
    }
}

impl<T: Write> Writable<T> for Service {
    fn write(&self, writer: &mut Writer<T>) -> Result<(), Error> {
        let element =
            start_ns_element(writer, XML_ELEM_SERVICE, XML_NS_SERVICE, None).map_err(xml_error)?;

        text_element(
            writer,
            XML_ELEM_SERVICE_TYPE,
            self.service_type.to_string().as_bytes(),
        )
        .map_err(xml_error)?;

        text_element(writer, XML_ELEM_SERVICE_ID, self.service_id.as_bytes()).map_err(xml_error)?;

        text_element(writer, XML_ELEM_SERVICE_SCPD_URL, self.scpd_url.as_bytes())
            .map_err(xml_error)?;

        text_element(
            writer,
            XML_ELEM_SERVICE_CONTROL_URL,
            self.control_url.as_bytes(),
        )
        .map_err(xml_error)?;

        text_element(
            writer,
            XML_ELEM_SERVICE_EVENT_URL,
            self.event_sub_url.as_bytes(),
        )
        .map_err(xml_error)?;

        element.end(writer).map_err(xml_error)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SpecVersion;
    use pretty_assertions::assert_eq;
    use std::str::from_utf8;

    /*
    <?xml version="1.0"?>
    <root xmlns="urn:schemas-upnp-org:device-1-0">
      <specVersion>
        <major>1</major>
        <minor>0</minor>
      </specVersion>
      <device>
        <deviceType>urn:schemas-upnp-org:device:Basic:1</deviceType>
        <friendlyName>AXIS P3301 - 00408CA45086</friendlyName>
        <manufacturer>AXIS</manufacturer>
        <manufacturerURL>http://www.axis.com/</manufacturerURL>
        <modelDescription>AXIS P3301 Network Fixed Dome Camera</modelDescription>
        <modelName>AXIS P3301</modelName>
        <modelNumber>P3301</modelNumber>
        <modelURL>http://www.axis.com/</modelURL>
        <serialNumber>00408CA45086</serialNumber>
        <UDN>uuid:Upnp-BasicDevice-1_0-00408CA45086</UDN>
        <serviceList>
          <service xmlns="urn:schemas-upnp-org:service-1-0">
            <serviceType>urn:axis-com:service:BasicService:1</serviceType>
            <serviceId>urn:axis-com:serviceId:BasicServiceId</serviceId>
            <controlURL>/upnp/control/BasicServiceId</controlURL>
            <eventSubURL>/upnp/event/BasicServiceId</eventSubURL>
            <SCPDURL>/scpd_basic.xml</SCPDURL>
          </service>
        </serviceList>
        <presentationURL>http://10.59.104.28:80/</presentationURL>
      </device>
      <URLBase>http://10.59.104.28:49152/</URLBase>
    </root>
    */

    const EX_DEVICE: &str = "<?xml version=\"1.0\"?><root xmlns=\"urn:schemas-upnp-org:device-1-0\"><specVersion><major>1</major><minor>0</minor></specVersion><URLBase>http://10.59.104.28:49152/</URLBase><device><deviceType>urn:schemas-upnp-org:device:Basic:1</deviceType><friendlyName>AXIS P3301 - 00408CA45086</friendlyName><manufacturer>AXIS</manufacturer><manufacturerURL>http://www.axis.com/</manufacturerURL><modelDescription>AXIS P3301 Network Fixed Dome Camera</modelDescription><modelName>AXIS P3301</modelName><modelNumber>P3301</modelNumber><modelURL>http://www.axis.com/</modelURL><serialNumber>00408CA45086</serialNumber><UDN>uuid:Upnp-BasicDevice-1_0-00408CA45086</UDN><serviceList><service xmlns=\"urn:schemas-upnp-org:service-1-0\"><serviceType>urn:axis-com:service:BasicService:1</serviceType><serviceId>urn:axis-com:serviceId:BasicServiceId</serviceId><SCPDURL>/scpd_basic.xml</SCPDURL><controlURL>/upnp/control/BasicServiceId</controlURL><eventSubURL>/upnp/event/BasicServiceId</eventSubURL></service></serviceList><presentationURL>http://10.59.104.28:80/</presentationURL></device></root>";

    #[test]
    fn test_xml_serialize() {
        let device = DeviceRoot {
            spec_version: SpecVersion::V10,
            url_base: "http://10.59.104.28:49152/".to_string(),
            device: Device {
                device_type: TypeID::new_device("Basic".to_string(), "1".to_string()),
                friendly_name: "AXIS P3301 - 00408CA45086".to_string(),
                manufacturer: "AXIS".to_string(),
                manufacturer_url: Some("http://www.axis.com/".to_string()),
                model_description: Some("AXIS P3301 Network Fixed Dome Camera".to_string()),
                model_name: "AXIS P3301".to_string(),
                model_number: Some("P3301".to_string()),
                model_url: Some("http://www.axis.com/".to_string()),
                serial_number: Some("00408CA45086".to_string()),
                unique_device_name: "uuid:Upnp-BasicDevice-1_0-00408CA45086".to_string(),
                upc: None,
                icon_list: vec![],
                service_list: vec![Service {
                    service_type: TypeID::new_service_with_domain(
                        "axis-com".to_string(),
                        "BasicService".to_string(),
                        "1".to_string(),
                    ),
                    service_id: "urn:axis-com:serviceId:BasicServiceId".to_string(),
                    scpd_url: "/scpd_basic.xml".to_string(),
                    control_url: "/upnp/control/BasicServiceId".to_string(),
                    event_sub_url: "/upnp/event/BasicServiceId".to_string(),
                }],
                device_list: vec![],
                presentation_url: Some("http://10.59.104.28:80/".to_string()),
            },
        };
        println!("\n{:#?}\n", device);
        let w = Vec::new();
        let written = to_writer(&device, w).unwrap();
        let xml = from_utf8(&written).unwrap();
        println!("{}\n\n", xml);

        assert_eq!(xml, EX_DEVICE);
    }
}
