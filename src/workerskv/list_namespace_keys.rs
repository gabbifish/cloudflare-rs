use super::Key;

use crate::endpoint::{Endpoint, Method};

/// List a Namespace's Keys
/// https://api.cloudflare.com/#workers-kv-namespace-list-a-namespace-s-keys
pub struct ListNamespaceKeys<'a> {
    pub account_identifier: &'a str,
    pub namespace_identifier: &'a str,
    pub params: ListNamespaceKeysParams,
}

impl<'a> Endpoint<Vec<Key>, ListNamespaceKeysParams> for ListNamespaceKeys<'a> {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        format!(
            "accounts/{}/storage/kv/namespaces/{}/keys",
            self.account_identifier, self.namespace_identifier
        )
    }
    fn query(&self) -> Option<ListNamespaceKeysParams> {
        Some(self.params.clone())
    }
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListNamespaceKeysParams {
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub prefix: Option<String>,
}
