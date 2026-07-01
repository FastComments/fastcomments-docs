## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|-----------|------------|
| tenant_id | String | Ναι |  |
| comment_id | String | Ναι |  |
| broadcast_id | String | Ναι |  |
| sso | String | Όχι |  |

## Απάντηση

Επιστρέφει: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'lock_comment Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = LockCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-9876".to_string(),
        broadcast_id: "news/article".to_string(),
        sso: Some("user-sso-token".to_string()),
    };
    let _resp = lock_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]