## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id | String | नहीं |  |
| page_size | i32 | नहीं |  |
| after_id | String | नहीं |  |
| include_context | bool | नहीं |  |
| after_created_at | i64 | नहीं |  |
| unread_only | bool | नहीं |  |
| dm_only | bool | नहीं |  |
| no_dm | bool | नहीं |  |
| include_translations | bool | नहीं |  |
| include_tenant_notifications | bool | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_my_notifications_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_user_notifications उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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