## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| namespace | String | Yes |  |
| component | String | Yes |  |
| locale | String | No |  |
| use_full_translation_ids | bool | No |  |

## 응답

반환: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_translations_response.rs)

## 예시

[inline-code-attrs-start title = 'get_translations 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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