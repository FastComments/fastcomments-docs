## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |

## 回應

回傳：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'delete_notification_count 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<FlagCommentPublic200Response, Error> {
    let params: DeleteNotificationCountParams = DeleteNotificationCountParams {
        tenant_id: String::from("acme-corp-tenant"),
        id: String::from("notification-9876"),
        user_id: Some(String::from("user-1234")),
    };
    let response: FlagCommentPublic200Response = delete_notification_count(configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---