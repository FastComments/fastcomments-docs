Aggregiert Dokumente, indem sie (wenn *groupBy* angegeben ist) gruppiert und mehrere Operationen angewendet werden. Verschiedene Operationen (z. B. sum, countDistinct, avg usw.) werden unterstützt.

## Parameters

| Name | Typ | Erforderlich | Beschreibung |
|------|------|---------------|--------------|
| tenant_id | String | Ja |  |
| aggregation_request | models::AggregationRequest | Ja |  |
| parent_tenant_id | String | Nein |  |
| include_stats | bool | Nein |  |

## Response

Rückgabe: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Beispiel

[inline-code-attrs-start title = 'Aggregat Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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