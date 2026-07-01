Agrège les documents en les regroupant (si groupBy est fourni) et en appliquant plusieurs opérations.  
Différentes opérations (par ex. sum, countDistinct, avg, etc.) sont prises en charge.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| tenant_id | String | Yes |  |
| aggregation_request | models::AggregationRequest | Yes |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Réponse

Retourne : [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple d\'agrégation'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---