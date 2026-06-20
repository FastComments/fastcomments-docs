## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| namespace | String | 是 |  |
| component | String | 是 |  |
| locale | String | 否 |  |
| use_full_translation_ids | bool | 否 |  |

## 回應

回傳: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## 範例

[inline-code-attrs-start title = 'get_translations 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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