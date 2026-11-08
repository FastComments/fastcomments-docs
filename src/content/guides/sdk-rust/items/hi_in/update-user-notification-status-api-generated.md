## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| notification_id | String | हाँ |  |
| new_status | String | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UpdateUserNotificationStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_status_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'update_user_notification_status उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_update() -> Result<(), Error> {
    let params = UpdateUserNotificationStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "news/article".to_string(),
        new_status: "read".to_string(),
        sso: Some("sso-token-123".to_string()),
    };
    let _response: UpdateUserNotificationStatusResponse =
        update_user_notification_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]