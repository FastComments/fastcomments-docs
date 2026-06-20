## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| text_search | String | 아니요 |  |
| by_ip_from_comment | String | 아니요 |  |
| filter | String | 아니요 |  |
| search_filters | String | 아니요 |  |
| demo | bool | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`ModerationApiCountCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_count_comments_response.rs)

## 예제

[inline-code-attrs-start title = 'get_count 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_count() -> Result<ModerationApiCountCommentsResponse, Error> {
    let params: GetCountParams = GetCountParams {
        text_search: Some("breaking election coverage".to_string()),
        by_ip_from_comment: Some("203.0.113.45".to_string()),
        filter: Some("status:approved".to_string()),
        search_filters: Some("section:politics tag:analysis".to_string()),
        demo: Some(false),
        sso: Some("acme-corp-tenant".to_string()),
    };
    let response: ModerationApiCountCommentsResponse = get_count(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---