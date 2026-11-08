## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id_ws | String | हाँ |  |
| user_ids | String | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_presence_statuses_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_user_presence_statuses उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetUserPresenceStatusesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id_ws: "news/article".to_string(),
        user_ids: "user123,user456".to_string(),
    };
    let _response = get_user_presence_statuses(&configuration, params).await?;
    Ok(())
}
[inline-code-end]