## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| namespace | String | 예 |  |
| component | String | 예 |  |
| locale | String | 아니요 |  |
| use_full_translation_ids | bool | 아니요 |  |

## 응답

반환: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## 예제

[inline-code-attrs-start title = 'get_translations 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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