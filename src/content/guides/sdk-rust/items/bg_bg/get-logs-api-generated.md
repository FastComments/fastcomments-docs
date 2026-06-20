---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| comment_id | String | Да |  |
| sso | String | Не |  |

## Отговор

Връща: [`ModerationApiGetLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_logs_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: GetLogsParams = GetLogsParams {
        comment_id: "news/article/2026/06/fastcomments-thread-12345".to_string(),
        sso: Some("acme-corp|user:john.doe@example.com".to_string()),
    };
    let logs: ModerationApiGetLogsResponse = get_logs(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---