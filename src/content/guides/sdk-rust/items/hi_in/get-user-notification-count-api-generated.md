## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetUserNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notification_count_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_user_notification_count उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetUserNotificationCountParams {
        tenant_id: "acme-corp-tenant".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let response = get_user_notification_count(&config, params).await?;
    println!("{:?}", response);
    Ok(())
}
[inline-code-end]