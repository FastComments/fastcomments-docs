## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| user_id | String | 아니요 |  |
| url_id | String | 아니요 |  |
| from_comment_id | String | 아니요 |  |
| viewed | bool | 아니요 |  |

## 응답

반환: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_notification_count 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_notification_count() -> Result<GetNotificationCount200Response, Error> {
    let params: GetNotificationCountParams = GetNotificationCountParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("user-12345")),
        url_id: Some(String::from("news/article/2026/product-launch")),
        from_comment_id: Some(String::from("cmt-000987")),
        viewed: Some(false),
    };
    let response: GetNotificationCount200Response = get_notification_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---