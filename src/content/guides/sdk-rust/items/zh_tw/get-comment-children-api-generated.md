## 參數

| 名稱 | 類型 | 是否必填 | 說明 |
|------|------|----------|-------------|
| comment_id | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳: [`ModerationApiChildCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_child_comments_response.rs)

## 範例

[inline-code-attrs-start title = 'get_comment_children 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_children() -> Result<ModerationApiChildCommentsResponse, Error> {
    let params: GetCommentChildrenParams = GetCommentChildrenParams {
        comment_id: "news/article-2026-06-19-cmt-42".to_string(),
        sso: Some("sso-token-user-8f3d2a".to_string()),
    };
    let children: ModerationApiChildCommentsResponse = get_comment_children(&configuration, params).await?;
    Ok(children)
}
[inline-code-end]

---