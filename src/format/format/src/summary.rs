fn process_summary_fields(summary_fields: &[SummaryField]) -> Vec<HashMap<String, StateResponseSummaryField>> {
  summary_fields
      .iter()
      .map(|summary_field| {
          let mut summary_map = HashMap::new();

          let key = match summary_field.summary_field_type.as_ref().map(|t| t.text.as_str()) {
              Some("OTHER") => summary_field.label_detection.as_ref().map_or("", |ld| ld.text.as_str()).to_string(),
              _ => summary_field.summary_field_type.as_ref().map_or("", |t| t.text.as_str()).to_string(),
          };

          let confidence_key = match summary_field.summary_field_type {
              Some(ref summary_field_type) => summary_field_type.confidence as f64, // Changed to f64
              None => summary_field.label_detection.as_ref().map_or(0.0, |ld| ld.confidence as f64), // Changed to f64
          };

          summary_map.insert(
              key,
              StateResponseSummaryField {
                  confidence_key,
                  confidence_value: summary_field.value_detection.confidence,
                  value: summary_field.value_detection.text.clone(),
              },
          );

          summary_map
      })
      .collect()
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateResponseSummaryField {
    confidence_key: f64,
    confidence_value: f64,
    value: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SummaryFieldsWrapper {
    summary_fields: Vec<SummaryField>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SummaryField {
    group_properties: Option<Vec<GroupProperty>>,
    page_number: i64,
    #[serde(rename = "Type")]
    summary_field_type: Option<TypeClass>,
    value_detection: Detection,
    label_detection: Option<Detection>,
}
