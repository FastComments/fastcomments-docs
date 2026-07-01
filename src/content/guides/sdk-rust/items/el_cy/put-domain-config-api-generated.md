## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenant_id | String | Ναι |  |
| domain_to_update | String | Ναι |  |
| update_domain_config_params | models::UpdateDomainConfigParams | Ναι |  |

## Απόκριση

Επιστρέφει: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/put_domain_config_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα put_domain_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_domain(configuration: &configuration::Configuration) -> Result<(), Error> {
    let update_params = models::UpdateDomainConfigParams {
        enable_comments: Some(true),
        moderation_level: Some("strict".to_string()),
        max_comment_length: Some(500),
        ..Default::default()
    };
    let params = PutDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news.example.com".to_string(),
        update_domain_config_params: update_params,
    };
    let _resp: PutDomainConfigResponse = put_domain_config(configuration, params).await?;
    Ok(())
}
[inline-code-end]