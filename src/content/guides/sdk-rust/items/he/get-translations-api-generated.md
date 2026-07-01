## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| namespace | String | Yes |  |
| component | String | Yes |  |
| locale | String | No |  |
| use_full_translation_ids | bool | No |  |

## תגובה

Returns: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## דוגמה

[inline-code-attrs-start title = 'דוגמת get_translations'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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