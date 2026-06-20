---
## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |

## Risposta

Restituisce: [`GetDomainConfigsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_domain_configs_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_domain_configs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_get_domain_configs_example() -> Result<GetDomainConfigsResponse, Error> {
    let params: GetDomainConfigsParams = GetDomainConfigsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_filter: Some("news.example.com".to_string()),
    };
    let response: GetDomainConfigsResponse = get_domain_configs(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---