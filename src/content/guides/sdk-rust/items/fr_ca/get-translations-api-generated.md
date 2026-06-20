## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| namespace | String | Oui |  |
| component | String | Oui |  |
| locale | String | Non |  |
| use_full_translation_ids | bool | Non |  |

## Réponse

Renvoie : [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_translations'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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