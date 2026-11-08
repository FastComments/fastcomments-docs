## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| namespace | String | हाँ |  |
| component | String | हाँ |  |
| locale | String | नहीं |  |
| use_full_translation_ids | bool | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_translations उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_translations() -> Result<(), Error> {
    let params = GetTranslationsParams {
        namespace: "acme-corp-tenant".to_string(),
        component: "news/article".to_string(),
        locale: Some("en-US".to_string()),
        use_full_translation_ids: Some(true),
    };
    let _response: GetTranslationsResponse = get_translations(&configuration, params).await?;
    Ok(())
}
[inline-code-end]