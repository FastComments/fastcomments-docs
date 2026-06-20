---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| comment_id | String | はい |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | はい |  |
| sso | String | いいえ |  |

## レスポンス

戻り値: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## 例

[inline-code-attrs-start title = 'un_block_comment_public の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: UnBlockCommentPublicParams = UnBlockCommentPublicParams {
    tenant_id: "acme-corp-tenant".to_string(),
    comment_id: "news/article/2026/06/19/cmt-987654".to_string(),
    public_block_from_comment_params: models::PublicBlockFromCommentParams {
        reason: "harassment".to_string(),
        duration_minutes: Some(1440),
        notify_author: Some(true),
    },
    sso: Some("sso-token-7f3b9a".to_string()),
};

let unblock_success: UnblockSuccess = un_block_comment_public(&configuration, params).await?;
[inline-code-end]

---