## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenant_id | String | Yes |  |
| domain_to_update | String | Yes |  |
| patch_domain_config_params | models::PatchDomainConfigParams | Yes |  |

## Απόκριση

Επιστρέφει: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_domain_config_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα patch_domain_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example() -> Result<(), Error> {
    let config = configuration::Configuration::default();
    let params = PatchDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news/article".to_string(),
        patch_domain_config_params: models::PatchDomainConfigParams {
            enable_comments: Some(true),
            theme: Some("dark".to_string()),
        },
    };
    let _response = patch_domain_config(&config, params).await?;
    Ok(())
}
[inline-code-end]