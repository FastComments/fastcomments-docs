## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| page | f64 | 否 |  |
| count | f64 | 否 |  |
| text_search | String | 否 |  |
| by_ip_from_comment | String | 否 |  |
| filters | String | 否 |  |
| search_filters | String | 否 |  |
| sorts | String | 否 |  |
| demo | bool | 否 |  |
| sso | String | 否 |  |

## 回應

回傳: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## 範例

[inline-code-attrs-start title = 'get_api_comments 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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