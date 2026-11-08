## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| page | f64 | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetHashTagsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_hash_tags_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_hash_tags उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetHashTagsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0),
    };
    let _response = get_hash_tags(&config, params).await?;
    Ok(())
}
[inline-code-end]