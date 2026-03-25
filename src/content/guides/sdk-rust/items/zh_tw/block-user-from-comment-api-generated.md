---
## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| id | String | 是 |  |
| block_from_comment_params | models::BlockFromCommentParams | 是 |  |
| user_id | String | 否 |  |
| anon_user_id | String | 否 |  |

## 回應

回傳：[`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_from_comment_public_200_response.rs)

## 範例

[inline-code-attrs-start title = 'block_user_from_comment 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: BlockUserFromCommentParams = BlockUserFromCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/article-2026-03-25/comment-842".to_string(),
        block_from_comment_params: models::BlockFromCommentParams {
            reason: Some("Repeated promotional links".to_string()),
            duration_minutes: Some(7_200),
            notify_user: Some(true),
        },
        user_id: Some("user-9812".to_string()),
        anon_user_id: None,
    };
    let response: BlockFromCommentPublic200Response = block_user_from_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---