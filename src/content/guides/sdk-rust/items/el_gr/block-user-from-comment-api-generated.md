## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| id | String | Ναι |  |
| block_from_comment_params | models::BlockFromCommentParams | Ναι |  |
| user_id | String | Όχι |  |
| anon_user_id | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/block_from_comment_public_200_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα block_user_from_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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