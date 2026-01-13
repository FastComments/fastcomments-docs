---
## 參數

| 名稱 | 類型 | 必需 | 說明 |
|------|------|------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_notification_count 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete_notification_count(configuration: &configuration::Configuration) -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteNotificationCountParams = DeleteNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "notification-9f8b7a6".to_string(),
    };
    let response: FlagCommentPublic200Response = delete_notification_count(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---