## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| namespace | String | Evet |  |
| component | String | Evet |  |
| locale | String | Hayır |  |
| use_full_translation_ids | bool | Hayır |  |

## Yanıt

Döndürür: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_translations Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_translations() -> Result<(), Error> {
    let params: GetTranslationsParams = GetTranslationsParams {
        namespace: "acme-corp-tenant".to_string(),
        component: "news/article".to_string(),
        locale: Some("en-US".to_string()),
        use_full_translation_ids: Some(true),
    };
    let translations: GetTranslationsResponse = get_translations(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---