## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| user_id | String | नहीं |  |
| limit | f64 | नहीं |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`ApiGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_list_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_user_badge_progress_list उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badge_progress(conf: &configuration::Configuration) -> Result<(), Error> {
    let params = GetUserBadgeProgressListParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: Some("user-98765".to_string()),
        limit: Some(20.0),
        skip: Some(5.0),
    };
    let _resp = get_user_badge_progress_list(conf, params).await?;
    Ok(())
}
[inline-code-end]