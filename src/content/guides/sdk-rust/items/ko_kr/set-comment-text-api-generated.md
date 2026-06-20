## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenant_id | String | 예 |  |
| comment_id | String | 예 |  |
| broadcast_id | String | 예 |  |
| comment_text_update_request | models::CommentTextUpdateRequest | 예 |  |
| edit_key | String | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`PublicApiSetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_set_comment_text_response.rs)

## 예제

[inline-code-attrs-start title = 'set_comment_text 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_comment() -> Result<PublicApiSetCommentTextResponse, Error> {
    let params: SetCommentTextParams = SetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-72f3a9".to_string(),
        broadcast_id: "news/article/2026/06/19/product-launch".to_string(),
        comment_text_update_request: models::CommentTextUpdateRequest {
            text: "Updated: Congratulations on the launch! Clarified a few points.".to_string(),
        },
        edit_key: Some("edit-key-9f8b".to_string()),
        sso: Some("sso-token-user-abc123".to_string()),
    };
    let response = set_comment_text(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---