## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| send_email | String | 否 |  |

## 回應

返回: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_moderator 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteModeratorParams {
        tenant_id: "acme-corp".to_string(),
        id: "moderator-123".to_string(),
        send_email: Some("admin@acme.com".to_string()),
    };
    let _ = delete_moderator(&configuration, params).await?;
    Ok(())
}
[inline-code-end]