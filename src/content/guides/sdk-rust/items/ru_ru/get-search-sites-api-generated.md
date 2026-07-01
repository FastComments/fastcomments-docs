## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| value | String | Нет |  |
| sso | String | Нет |  |

## Ответ

Возвращает: [`ModerationSiteSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_site_search_response.rs)

## Пример

[inline-code-attrs-start title = 'get_search_sites Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetSearchSitesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("news/article".to_string()),
        sso: Some("sso-token-abc".to_string()),
    };
    let _response = get_search_sites(&config, params).await?;
    Ok(())
}
[inline-code-end]

---