## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| comment_id | String | Tak |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`ModerationApiGetLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_logs_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_logs'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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