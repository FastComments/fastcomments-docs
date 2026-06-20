## Παράμετροι

| Όνομα | Type | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| comment_id | String | Ναι |  |
| broadcast_id | String | Ναι |  |
| edit_key | String | Όχι |  |
| sso | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`PublicApiDeleteCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/public_api_delete_comment_response.rs)

## Παράδειγμα

[inline-code-attrs-start title = 'delete_comment_public Παράδειγμα'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteCommentPublicParams = DeleteCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("cmt-7f3a2b9"),
        broadcast_id: String::from("news/article/2026/06/19/article-12345"),
        edit_key: Some(String::from("editkey-9d2f")),
        sso: Some(String::from("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")),
    };
    let response: PublicApiDeleteCommentResponse = delete_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---