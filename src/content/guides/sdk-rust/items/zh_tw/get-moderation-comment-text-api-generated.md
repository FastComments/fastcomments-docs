## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| sso | String | No |  |

## 回應

返回：[`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comment_text_response.rs)

## 範例

[inline-code-attrs-start title = '取得審核評論文字 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetModerationCommentTextParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _response: GetCommentTextResponse =
        get_moderation_comment_text(&configuration, params).await?;
    Ok(())
}
[inline-code-end]