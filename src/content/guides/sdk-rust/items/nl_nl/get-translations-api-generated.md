---
## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| namespace | String | Yes |  |
| component | String | Yes |  |
| locale | String | No |  |
| use_full_translation_ids | bool | No |  |

## Respons

Retourneert: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_translations Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---