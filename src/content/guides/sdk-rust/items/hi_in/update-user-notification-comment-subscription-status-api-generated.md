किसी विशिष्ट कमेंट के लिए नोटिफिकेशन सक्षम या अक्षम करें।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| notification_id | String | हाँ |  |
| opted_in_or_out | String | हाँ |  |
| comment_id | String | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`UpdateUserNotificationCommentSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_comment_subscription_status_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'update_user_notification_comment_subscription_status उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = UpdateUserNotificationCommentSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_string(),
        notification_id: "comment-reply".to_string(),
        opted_in_or_out: "opted_in".to_string(),
        comment_id: "12345".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _response = update_user_notification_comment_subscription_status(configuration, params).await?;
    Ok(())
}
[inline-code-end]