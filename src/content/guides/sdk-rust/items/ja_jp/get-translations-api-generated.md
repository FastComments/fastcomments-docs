## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| namespace | String | はい |  |
| component | String | はい |  |
| locale | String | いいえ |  |
| use_full_translation_ids | bool | いいえ |  |

## レスポンス

戻り値: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## 例

[inline-code-attrs-start title = 'get_translations の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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