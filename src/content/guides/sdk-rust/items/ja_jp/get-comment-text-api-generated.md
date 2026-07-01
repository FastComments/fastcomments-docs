## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| edit_key | String | いいえ |  |
| sso | String | いいえ |  |

## レスポンス

返却: [`PublicApiGetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_get_comment_text_response.rs)

## 例

[inline-code-attrs-start title = 'get_comment_text の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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