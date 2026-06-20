## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| page | f64 | 아니요 |  |
| count | f64 | 아니요 |  |
| text_search | String | 아니요 |  |
| by_ip_from_comment | String | 아니요 |  |
| filters | String | 아니요 |  |
| search_filters | String | 아니요 |  |
| sorts | String | 아니요 |  |
| demo | bool | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## 예제

[inline-code-attrs-start title = 'get_api_comments 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params: GetApiCommentsParams = GetApiCommentsParams {
        page: Some(1.0),
        count: Some(20.0),
        text_search: Some(String::from("breaking election results")),
        by_ip_from_comment: Some(String::from("203.0.113.45")),
        filters: Some(String::from("status:approved,thread:news/article")),
        search_filters: Some(String::from("author:john.doe@example.com")),
        sorts: Some(String::from("-created_at")),
        demo: Some(false),
        sso: Some(String::from("acme-corp-tenant")),
    };
    let response: ModerationApiGetCommentsResponse = get_api_comments(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---