## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| value | String | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_page_search_response.rs)

## 예제

[inline-code-attrs-start title = 'get_search_pages 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetSearchPagesParams {
        value: Some("news/article/world/2026-summit".to_string()),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let moderation_response: ModerationPageSearchResponse =
        get_search_pages(&configuration, params).await?;
    let _status: ApiStatus = moderation_response.status;
    Ok(())
}
[inline-code-end]

---