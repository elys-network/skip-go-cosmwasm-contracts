use cosmwasm_std::{Binary, ContractResult, Querier, QuerierResult, SystemError, SystemResult};

pub struct MockQuerier {
    responses: Vec<(String, Binary)>,
}

impl MockQuerier {
    pub fn new() -> Self {
        Self { responses: vec![] }
    }

    pub fn mock_stargate_response(&mut self, response: (String, Binary)) {
        self.responses.push(response);
    }
}

impl Default for MockQuerier {
    fn default() -> Self {
        Self::new()
    }
}

impl Querier for MockQuerier {
    fn raw_query(&self, query_raw: &[u8]) -> QuerierResult {
        let query = String::from_utf8(query_raw.to_vec()).unwrap();

        // Find matching response for query
        for (query_type, response) in &self.responses {
            if query.contains(query_type) {
                return SystemResult::Ok(ContractResult::Ok(response.clone()));
            }
        }

        SystemResult::Err(SystemError::UnsupportedRequest {
            kind: "no matching response found".to_string(),
        })
    }
}
