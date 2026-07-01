Συγκεντρώνει έγγραφα ομαδοποιώντας τα (εάν παρέχεται groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.  
Διαφορετικές λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.) υποστηρίζονται.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| aggregation_request | models::AggregationRequest | Yes |  |
| parent_tenant_id | String | No |  |
| include_stats | bool | No |  |

## Απάντηση

Επιστρέφει: [`AggregateResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/aggregate_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα Συγκέντρωσης'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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