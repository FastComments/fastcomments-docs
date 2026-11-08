## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| id | String | हाँ |  |
| user_id | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`DeleteSubscriptionApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_subscription_api_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'delete_subscription उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = DeleteSubscriptionParams {
        tenant_id: "acme-corp".to_string(),
        id: "sub-2024-09".to_string(),
        user_id: Some("user-42".to_string()),
    };
    let _response = delete_subscription(&config, params).await?;
    Ok(())
}
[inline-code-end]