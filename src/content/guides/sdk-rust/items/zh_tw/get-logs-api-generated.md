## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳: [`ModerationApiGetLogsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_logs_response.rs)

## 範例

[inline-code-attrs-start title = 'get_logs 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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