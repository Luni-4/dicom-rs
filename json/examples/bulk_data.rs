/// Example demonstrating BulkDataDicomObject for handling DICOM JSON with BulkDataURIs
///
/// This example shows how to:
/// 1. Deserialize DICOM JSON containing BulkDataURI fields
/// 2. Access the bulk data URI references
/// 3. Access the regular DICOM data elements
/// 4. Serialize the object back to DICOM JSON

use dicom_core::Tag;
use dicom_json::BulkDataDicomObject;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example DICOM JSON with both regular data and bulk data references
    let json_str = r#"{
        "00080016": {
            "vr": "UI",
            "Value": ["1.2.840.10008.5.1.4.1.1.2"]
        },
        "00080018": {
            "vr": "UI",
            "Value": ["1.2.3.4.5.6.7.8.9"]
        },
        "00100010": {
            "vr": "PN",
            "Value": [{
                "Alphabetic": "Doe^John"
            }]
        },
        "00100020": {
            "vr": "LO",
            "Value": ["12345"]
        },
        "7FE00010": {
            "vr": "OW",
            "BulkDataURI": "http://example.com/studies/1.2.3/series/4.5.6/instances/7.8.9/bulkdata/pixeldata"
        }
    }"#;

    println!("=== Deserializing DICOM JSON with BulkDataURI ===\n");

    // Deserialize to BulkDataDicomObject instead of InMemDicomObject
    let obj: BulkDataDicomObject = dicom_json::from_str(json_str)?;

    println!("Successfully deserialized DICOM JSON");
    println!();

    // Access regular data elements
    println!("=== Regular Data Elements ===");
    let sop_class_uid_tag = Tag(0x0008, 0x0016);
    if let Some(elem) = obj.object().get(sop_class_uid_tag) {
        println!("SOP Class UID: {}", elem.to_str().unwrap_or_default());
    }

    let patient_name_tag = Tag(0x0010, 0x0010);
    if let Some(elem) = obj.object().get(patient_name_tag) {
        println!("Patient Name: {}", elem.to_str().unwrap_or_default());
    }

    let patient_id_tag = Tag(0x0010, 0x0020);
    if let Some(elem) = obj.object().get(patient_id_tag) {
        println!("Patient ID: {}", elem.to_str().unwrap_or_default());
    }
    println!();

    // Access bulk data URIs
    println!("=== Bulk Data References ===");
    let pixel_data_tag = Tag(0x7FE0, 0x0010);
    if let Some(uri) = obj.bulk_data_uri(&pixel_data_tag) {
        println!("Pixel Data URI: {}", uri);
    }
    println!();

    // Show all bulk data tags
    println!("=== All Tags with Bulk Data URIs ===");
    for tag in obj.bulk_data_tags() {
        if let Some(uri) = obj.bulk_data_uri(tag) {
            println!("Tag {}: {}", tag, uri);
        }
    }
    println!();

    // Serialize back to JSON
    println!("=== Serialized back to JSON ===");
    let serialized_json = dicom_json::to_string_pretty(&obj)?;
    println!("{}", serialized_json);

    Ok(())
}
