## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| page | f64 | 아니오 |  |
| count | f64 | 아니오 |  |
| text_search | String | 아니오 |  |
| by_ip_from_comment | String | 아니오 |  |
| filters | String | 아니오 |  |
| search_filters | String | 아니오 |  |
| sorts | String | 아니오 |  |
| demo | bool | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`ModerationApiGetCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_get_comments_response.rs)

## 예시

[inline-code-attrs-start title = 'get_api_comments 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params = GetApiCommentsParams {
        tenant_id: "acme-corp-tenant".to_string(),
        page: Some(1.0),
        count: Some(20.0),
        text_search: Some("rust".to_string()),
        by_ip_from_comment: None,
        filters: Some("status:approved".to_string()),
        search_filters: Some("author:john".to_string()),
        sorts: Some("date:desc".to_string()),
        demo: Some(false),
        sso: None,
    };
    let _response = get_api_comments(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---