
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutputData{
    blocks: Vec<StateResponseBlock>, 
    line_item_groups: Vec<StateResponseLineItemGroup>, 
    summary_fields:  Vec<HashMap<String, StateResponseSummaryField>>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponse {
    document_metadata: DocumentMetadata,
    expense_documents: Vec<StateResponseExpenseDocument>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Detection {
    confidence: f64,
    geometry: Geometry,
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GroupProperty {
    id: String,
    types: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StateResponseExpenseDocument {
    blocks: Vec<StateResponseBlock>,
    expense_index: i64,
    line_item_groups: Vec<LineItemGroup>,
    summary_fields: Vec<HashMap<String, StateResponseSummaryField>>,
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DocumentMetadata {
    pages: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpenseDocument {
    blocks: Vec<Block>,
    expense_index: i64,
    line_item_groups: Vec<LineItemGroup>,
    summary_fields: Vec<SummaryField>,
}