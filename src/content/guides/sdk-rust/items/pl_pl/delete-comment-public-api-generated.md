## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| broadcast_id | String | Tak |  |
| edit_key | String | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/delete_comment_public_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'delete_comment_public Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: DeleteCommentPublicParams = DeleteCommentPublicParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_id: String::from("news/article/2026/03/interesting-story#cmt-67890"),
        broadcast_id: String::from("news-article-12345"),
        edit_key: Some(String::from("editkey-3f2b9a")),
        sso: Some(String::from("sso-jwt-eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")),
    };
    let response: DeleteCommentPublic200Response = delete_comment_public(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---