## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| url_id | String | 아니오 |  |
| page_size | i32 | 아니오 |  |
| after_id | String | 아니오 |  |
| include_context | bool | 아니오 |  |
| after_created_at | i64 | 아니오 |  |
| unread_only | bool | 아니오 |  |
| dm_only | bool | 아니오 |  |
| no_dm | bool | 아니오 |  |
| include_translations | bool | 아니오 |  |
| include_tenant_notifications | bool | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## 예시

[inline-code-attrs-start title = 'get_user_notifications 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: Some("news/article".to_string()),
        page_size: Some(20),
        after_id: None,
        include_context: Some(true),
        after_created_at: None,
        unread_only: Some(false),
        dm_only: Some(false),
        no_dm: Some(true),
        include_translations: Some(false),
        include_tenant_notifications: Some(true),
        sso: None,
    };
    let _resp = get_user_notifications(&config, params).await?;
    Ok(())
}
[inline-code-end]