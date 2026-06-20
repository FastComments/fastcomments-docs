## Παράμετροι

| Όνομα | Type | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| namespace | String | Ναι |  |
| component | String | Ναι |  |
| locale | String | Όχι |  |
| use_full_translation_ids | bool | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_translations'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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