## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| comment_id | String | はい |  |
| sso | String | いいえ |  |

## レスポンス

返り値: [`ModerationApiChildCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_api_child_comments_response.rs)

## 例

[inline-code-attrs-start title = 'get_comment_children の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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