## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| after_id | String | नहीं |  |
| after_created_at | i64 | नहीं |  |
| unread_only | bool | नहीं |  |
| dm_only | bool | नहीं |  |
| no_dm | bool | नहीं |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'reset_user_notifications उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = ResetUserNotificationsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        after_id: Some("notif-12345".to_string()),
        after_created_at: Some(1_640_995_200),
        unread_only: Some(true),
        dm_only: Some(false),
        no_dm: Some(true),
        sso: Some("sso-provider".to_string()),
    };
    let _response: ResetUserNotificationsResponse =
        reset_user_notifications(&configuration, params).await?;
    Ok(())
}
[inline-code-end]