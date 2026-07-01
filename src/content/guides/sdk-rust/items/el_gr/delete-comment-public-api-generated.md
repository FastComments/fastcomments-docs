## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenant_id | String | Ναι |  |
| comment_id | String | Ναι |  |
| broadcast_id | String | Ναι |  |
| edit_key | String | Όχι |  |
| sso | String | Όχι |  |

## Απάντηση

Επιστρέφει: [`PublicApiDeleteCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_delete_comment_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'delete_comment_public Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_delete() -> Result<(), Error> {
    let params = DeleteCommentPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        broadcast_id: "news/article-6789".to_string(),
        edit_key: Some("edit-abc123".to_string()),
        sso: Some("sso-token-xyz".to_string()),
    };
    let response = delete_comment_public(&configuration, params).await?;
    let _deleted: PublicApiDeleteCommentResponse = response;
    Ok(())
}
[inline-code-end]