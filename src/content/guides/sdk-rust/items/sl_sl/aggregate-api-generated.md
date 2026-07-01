Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.  
Različne operacije (npr. sum, countDistinct, avg itd.) so podprte.

## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|------|
| tenant_id | String | Yes |  |
| aggregation_request | models::AggregationRequest | Yes |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Odgovor

Vrne: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer agregacije'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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