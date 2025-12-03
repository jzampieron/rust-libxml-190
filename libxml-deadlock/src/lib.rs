use libxml::error::StructuredError;
use libxml::parser::Parser as XmlParser;
use libxml::schemas::SchemaParserContext;
use libxml::schemas::SchemaValidationContext;

/// # Errors
/// 
/// Will return error if the XSD fails to parse or if the file cannot be found.
/// 
pub fn load_schema(schema_file: &str) -> Result<SchemaValidationContext, Vec<StructuredError>> {
    // Parse the XSD schema
    let mut schema_parser = SchemaParserContext::from_file(schema_file);
    
    SchemaValidationContext::from_parser(&mut schema_parser)
}

pub fn validate_xml_buf_with_parser_schema(parser: &XmlParser, xml_buf: &[u8], xsd: &mut SchemaValidationContext) -> bool {
    // Parse the XML file
    parser.parse_string(xml_buf)
        .map( |d| xsd.validate_document(&d) ).is_ok()        
}

/// # Panics
/// 
/// Cannot actually panic b/c we guard against it.
pub fn validate_xml_buf_with_schema(xml_buf: &[u8], xsd: &mut SchemaValidationContext) -> bool {
    // Parse the XML file
    let parser = XmlParser::default();
    let doc = parser.parse_string(xml_buf);
    
    // Validate the XML file against the schema
    doc.is_ok() && xsd.validate_document(&doc.unwrap()).is_ok()
}

/// # Panics
/// 
/// Cannot actually panic b/c we guard against it.
pub fn validate_xml_str_with_schema(xml_buf: &str, xsd: &mut SchemaValidationContext) -> bool {
    // Parse the XML file
    let parser = XmlParser::default();
    let doc = parser.parse_string(xml_buf.as_bytes());
    
    // Validate the XML file against the schema
    doc.is_ok() && xsd.validate_document(&doc.unwrap()).is_ok()
}

/// # Panics
/// 
/// Cannot actually panic b/c we guard against it.
pub fn validate_xml_with_schema(xml_file: &str, xsd: &mut SchemaValidationContext) -> bool {
    // Parse the XML file
    let parser = XmlParser::default();
    let doc = parser.parse_file(xml_file);
    
    // Validate the XML file against the schema
    doc.is_ok() && xsd.validate_document(&doc.unwrap()).is_ok()
}

/// # Panics
/// 
/// Cannot actually panic b/c we guard against it.
#[must_use] pub fn validate_xml(xml_file: &str, schema_file: &str) -> bool {
    let xsd = load_schema(schema_file);
    if xsd.is_err() {
        return false; 
    }
    let mut xsd = xsd.unwrap();

    validate_xml_with_schema(xml_file, &mut xsd)
}

// DANGER: You must use --test-threads=1 and hope for the best here. 
// See 
// - https://github.com/rust-lang/rust/issues/104053
// - https://github.com/KWARC/rust-libxml/issues/190
// The way tests run in rust by default break the libxml2
// initialization requirement: https://dev.w3.org/XInclude-Test-Suite/libxml2-2.4.24/libxml2-2.4.24/doc/threads.html

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_FILES: &[&str] = &[
        "Test0.xml",
        "Test1.xml",
        "Test2.xml",
        "Test3.xml",
        "Test4.xml",
        "Test5.xml",
        "Test6.xml",
        "Test7.xml",
        "Test8.xml",
        "Test9.xml",
    ];

    const SCHEMA_PATH: &str = "/ExampleSchema.xsd";

    const TEST_FILE_PATH: &str = "/sample_data/";

    #[test]
    fn test_load_schema() {
        let m_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        println!("CARGO_MANIFEST_DIR: {m_dir}");

        let test_schema_fn = m_dir.clone() + SCHEMA_PATH;

        // Parse the XSD schema
        let xsd = load_schema(&test_schema_fn);
   
        assert!(xsd.is_ok());

        let mut xsd = xsd.unwrap();

        let base_path = m_dir.clone() + TEST_FILE_PATH;

        for a_test_file in TEST_FILES {
            let test_data = base_path.clone() + a_test_file;
            assert!(validate_xml_buf_with_schema(&std::fs::read(test_data).unwrap(), &mut xsd));
        }
    }

    #[test]
    fn test_validate_buf_with_schema() {
        let m_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        println!("CARGO_MANIFEST_DIR: {m_dir}");

        let test_schema = m_dir.clone() + SCHEMA_PATH;

        // Parse the XSD schema
        let mut schema_parser = SchemaParserContext::from_file(&test_schema);
        let xsd = SchemaValidationContext::from_parser(&mut schema_parser);
   
        assert!(xsd.is_ok());

        let mut xsd = xsd.unwrap();

        let base_path = m_dir.clone() + TEST_FILE_PATH;

        for a_test_file in TEST_FILES {
            let test_data = base_path.clone() + a_test_file;
            assert!(validate_xml_buf_with_schema(&std::fs::read(test_data).unwrap(), &mut xsd));
        }
    }

    #[test]
    fn test_validate_with_schema() {
        let m_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        println!("CARGO_MANIFEST_DIR: {m_dir}");

        let test_schema = m_dir.clone() + SCHEMA_PATH;

        // Parse the XSD schema
        let mut schema_parser = SchemaParserContext::from_file(&test_schema);
        let xsd = SchemaValidationContext::from_parser(&mut schema_parser);
   
        assert!(xsd.is_ok());

        let mut xsd = xsd.unwrap();

        let base_path = m_dir.clone() + TEST_FILE_PATH;

        for a_test_file in TEST_FILES {
            let test_data = base_path.clone() + a_test_file;
            assert!(validate_xml_with_schema(&test_data, &mut xsd));
        }

    }

    #[test]
    fn test_validate() {
        let m_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        println!("CARGO_MANIFEST_DIR: {m_dir}");

        let test_schema = m_dir.clone() + SCHEMA_PATH;
    
        let base_path = m_dir.clone() + TEST_FILE_PATH;

        for a_test_file in TEST_FILES {
            let test_data = base_path.clone() + a_test_file;
            assert!(validate_xml(&test_data, &test_schema));
        }
    }
}