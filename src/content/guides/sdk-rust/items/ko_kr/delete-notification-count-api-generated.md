## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/flag_comment_public_200_response.rs)

## 예제

[inline-code-attrs-start title = 'delete_notification_count 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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