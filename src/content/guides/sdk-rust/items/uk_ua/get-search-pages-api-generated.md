## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| value | String | Ні |  |
| sso | String | Ні |  |

## Відповідь

Повертає: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_page_search_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_search_pages'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetSearchPagesParams {
        value: Some("news/article/world/2026-summit".to_string()),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let moderation_response: ModerationPageSearchResponse =
        get_search_pages(&configuration, params).await?;
    let _status: ApiStatus = moderation_response.status;
    Ok(())
}
[inline-code-end]

---