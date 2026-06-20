---
특정 댓글에 대한 알림을 활성화하거나 비활성화합니다.

## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| notification_id | String | 예 |  |
| opted_in_or_out | String | 예 |  |
| comment_id | String | 예 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## 예제

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<UpdateUserNotificationCommentSubscriptionStatusResponse, Error> {
    let params: UpdateUserNotificationCommentSubscriptionStatusParams = UpdateUserNotificationCommentSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "news/winter-2026-update".to_string(),
        opted_in_or_out: "opted_in".to_string(),
        comment_id: "article-42-comment-7".to_string(),
        sso: Some("user-123|eyJhbGciOi...".to_string()),
    };
    let response: UpdateUserNotificationCommentSubscriptionStatusResponse =
        update_user_notification_comment_subscription_status(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---