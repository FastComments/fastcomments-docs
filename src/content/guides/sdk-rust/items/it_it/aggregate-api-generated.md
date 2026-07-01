Aggrega documenti raggruppandoli (se groupBy è fornito) e applicando più operazioni.  
Sono supportate diverse operazioni (ad es. sum, countDistinct, avg, ecc.).

## Parameters

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| aggregation_request | models::AggregationRequest | Sì |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Response

Returns: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di aggregazione'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let aggregation_request = models::AggregationRequest::default();
    let params = AggregateParams {
        tenant_id: "acme-corp-tenant".to_string(),
        aggregation_request,
        parent_tenant_id: Some("global-tenant".to_string()),
        include_stats: Some(true),
    };
    let _response = aggregate(&config, params).await?;
    Ok(())
}
[inline-code-end]