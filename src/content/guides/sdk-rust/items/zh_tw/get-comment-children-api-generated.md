## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | Yes |  |
| comment_id | String | Yes |  |
| sso | String | No |  |

## 回應

返回: [`ModerationApiChildCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_child_comments_response.rs)

## 範例

[inline-code-attrs-start title = 'get_comment_children 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_children(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetCommentChildrenParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/article/comment-9876".to_string(),
        sso: Some("user-42".to_string()),
    };
    let _response: ModerationApiChildCommentsResponse = get_comment_children(config, params).await?;
    Ok(())
}
[inline-code-end]