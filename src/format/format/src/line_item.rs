fn process_line_item_groups(line_item_groups: &Vec<LineItemGroup>) -> Vec<StateResponseLineItemGroup> {
  line_item_groups.iter().map(|line_item_group| {
      StateResponseLineItemGroup {
          line_item_group_index: line_item_group.line_item_group_index,
          line_items: line_item_group.line_items.iter().map(|line_item| {
              let line_item_expense_fields: Vec<StateResponseLineItemExpenseField> = line_item.line_item_expense_fields.iter().map(|field| {
                  let mut expense_field = StateResponseLineItemExpenseField {
                      product_code: None,
                      item: None,
                      expense_row: None,
                      price: None
                  };
                  match field.line_item_expense_field_type.text.as_str()  {
                      "PRODUCT_CODE" =>  expense_field.product_code = Some(StateResponseSummaryField {
                          confidence_key: field.line_item_expense_field_type.confidence,
                          confidence_value: field.value_detection.confidence,
                          value: field.value_detection.text.clone(),
                      }),
                      "ITEM" =>  expense_field.item = Some(StateResponseSummaryField {
                          confidence_key: field.line_item_expense_field_type.confidence,
                          confidence_value: field.value_detection.confidence,
                          value: field.value_detection.text.clone(),
                      }),
                      "EXPENSE_ROW" =>  expense_field.expense_row = Some(StateResponseSummaryField {
                          confidence_key: field.line_item_expense_field_type.confidence,
                          confidence_value: field.value_detection.confidence,
                          value: field.value_detection.text.clone(),
                      }),
                      "PRICE" =>  expense_field.price = Some(StateResponseSummaryField {
                          confidence_key: field.line_item_expense_field_type.confidence,
                          confidence_value: field.value_detection.confidence,
                          value: field.value_detection.text.clone(),
                      }),
                      _ => {} // Handling other cases if needed
                  }
                  expense_field
              }).collect();
              StateResponseLineItem { line_item_expense_fields }
          }).collect()
      }
  }).collect()
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItemGroup {
    #[serde(rename = "LineItemGroupIndex")]
    line_item_group_index: i64,
    line_items: Vec<LineItem>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItem {
    line_item_expense_fields: Vec<LineItemExpenseField>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LineItemExpenseField {
    page_number: i64,
    #[serde(rename = "Type")]
    line_item_expense_field_type: TypeClass,
    value_detection: Detection,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseLineItemGroup {
    line_item_group_index: i64,
    line_items: Vec<StateResponseLineItem>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseLineItem {
    line_item_expense_fields: Vec<StateResponseLineItemExpenseField>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct StateResponseLineItemExpenseField {
    product_code: Option<StateResponseSummaryField>,
    item: Option<StateResponseSummaryField>,
    price: Option<StateResponseSummaryField>,
    expense_row: Option<StateResponseSummaryField>,
}
