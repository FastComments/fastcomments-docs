---
## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| comment_id | String | Ναι |  |
| public_block_from_comment_params | models::PublicBlockFromCommentParams | Ναι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/unblock_success.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα un_block_comment_public'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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