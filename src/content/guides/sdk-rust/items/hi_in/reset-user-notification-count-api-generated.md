## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/reset_user_notifications_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'reset_user_notification_count उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(config: &configuration::Configuration) -> Result<(), Error> {
    let params = ResetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("john.doe".to_string()),
    };
    let _response: ResetUserNotificationsResponse = reset_user_notification_count(config, params).await?;
    Ok(())
}
[inline-code-end]