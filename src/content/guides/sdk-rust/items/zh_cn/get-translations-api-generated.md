## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| namespace | String | 是 |  |
| component | String | 是 |  |
| locale | String | 否 |  |
| use_full_translation_ids | bool | 否 |  |

## 响应

返回： [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## 示例

[inline-code-attrs-start title = 'get_translations 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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