## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| edit_key | String | No |  |
| sso | String | No |  |

## 응답

반환: [`PublicApiGetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_get_comment_text_response.rs)

## 예시

[inline-code-attrs-start title = 'get_comment_text 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        edit_key: Some("edit-key-abc".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let _response = get_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]