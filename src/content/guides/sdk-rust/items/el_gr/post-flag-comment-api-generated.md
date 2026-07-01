## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenant_id | String | Ναι |  |
| comment_id | String | Ναι |  |
| broadcast_id | String | Όχι |  |
| sso | String | Όχι |  |

## Απάντηση

Επιστρέφει: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'post_flag_comment Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn flag_comment_example() -> Result<(), Error> {
    let params = PostFlagCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-9f8e7d".to_string(),
        broadcast_id: Some("broadcast-2024-01".to_string()),
        sso: Some("sso-uid-12345".to_string()),
    };
    post_flag_comment(&config, params).await?;
    Ok(())
}
[inline-code-end]