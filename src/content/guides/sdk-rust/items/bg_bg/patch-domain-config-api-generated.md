## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| domain_to_update | String | Да |  |
| patch_domain_config_params | models::PatchDomainConfigParams | Да |  |

## Отговор

Връща: [`PatchDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_domain_config_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за patch_domain_config'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: PatchDomainConfigParams = PatchDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news/article".to_string(),
        patch_domain_config_params: models::PatchDomainConfigParams {
            allowed_origins: Some(vec![
                "https://www.acme.com".to_string(),
                "https://blog.acme.com".to_string(),
            ]),
            enable_moderation: Some(true),
            moderation_mode: Some("pre".to_string()),
            webhook_url: Some("https://hooks.acme.com/comments".to_string()),
            max_comment_length: Some(1000),
        },
    };
    let response: PatchDomainConfigResponse = patch_domain_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---