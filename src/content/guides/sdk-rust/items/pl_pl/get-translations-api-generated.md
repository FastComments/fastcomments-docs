## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| namespace | String | Yes |  |
| component | String | Yes |  |
| locale | String | No |  |
| use_full_translation_ids | bool | No |  |

## Odpowiedź

Zwraca: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_translations'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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