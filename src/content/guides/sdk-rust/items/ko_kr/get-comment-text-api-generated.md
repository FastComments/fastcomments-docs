## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 예 |  |
| edit_key | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_200_response.rs)

## 예제

[inline-code-attrs-start title = 'get_comment_text 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
pub async fn run() -> Result<GetCommentText200Response, Error> {
    let params: GetCommentTextParams = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article-2026-03-25-98765".to_string(),
        edit_key: Some("edit_4f3d2b9a".to_string()),
        sso: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".to_string()),
    };
    let comment: GetCommentText200Response = get_comment_text(&configuration, params).await?;
    Ok(comment)
}
[inline-code-end]

---