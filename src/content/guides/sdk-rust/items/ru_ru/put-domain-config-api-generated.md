## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| domain_to_update | String | Да |  |
| update_domain_config_params | models::UpdateDomainConfigParams | Да |  |

## Ответ

Возвращает: [`PutDomainConfigResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/put_domain_config_response.rs)

## Пример

[inline-code-attrs-start title = 'put_domain_config Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_domain_config_example() -> Result<(), Error> {
    let update_params: models::UpdateDomainConfigParams = models::UpdateDomainConfigParams {
        enable_comments: Some(true),
        moderation_mode: Some("pre_moderation".to_string()),
        allowed_origins: Some(vec![
            "https://news.example.com".to_string(),
            "https://www.news.example.com".to_string(),
        ]),
        require_https: Some(true),
        max_comment_length: Some(1000),
    };

    let params: PutDomainConfigParams = PutDomainConfigParams {
        tenant_id: "acme-corp-tenant".to_string(),
        domain_to_update: "news.example.com".to_string(),
        update_domain_config_params: update_params,
    };

    let response: PutDomainConfigResponse = put_domain_config(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---