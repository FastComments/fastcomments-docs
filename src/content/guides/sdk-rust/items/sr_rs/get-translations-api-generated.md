## Parameters

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| namespace | String | Da |  |
| component | String | Da |  |
| locale | String | Ne |  |
| use_full_translation_ids | bool | Ne |  |

## Response

Vraća: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## Primer

[inline-code-attrs-start title = 'get_translations Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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