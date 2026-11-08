## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| skip | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_moderators_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_moderators उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetModeratorsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        skip: Some(20.0),
    };
    let _response = get_moderators(&configuration, params).await?;
    Ok(())
}
[inline-code-end]