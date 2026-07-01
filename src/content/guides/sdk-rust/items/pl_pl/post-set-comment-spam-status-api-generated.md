## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| spam | bool | Nie |  |
| perm_not_spam | bool | Nie |  |
| broadcast_id | String | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`ApiEmptyResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_empty_response.rs)

## Przykład

[inline-code-attrs-start title = 'post_set_comment_spam_status Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = PostSetCommentSpamStatusParams {
        tenant_id: "acme-corp-tenant".into(),
        comment_id: "comment-12345".into(),
        spam: Some(true),
        perm_not_spam: Some(false),
        broadcast_id: Some("broadcast-678".into()),
        sso: Some("user@example.com".into()),
    };
    post_set_comment_spam_status(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---