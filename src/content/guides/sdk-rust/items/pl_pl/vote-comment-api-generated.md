## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| comment_id | String | Tak |  |
| url_id | String | Tak |  |
| broadcast_id | String | Tak |  |
| vote_body_params | models::VoteBodyParams | Tak |  |
| session_id | String | Nie |  |
| sso | String | Nie |  |

## Odpowiedź

Zwraca: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład vote_comment'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: VoteCommentParams = VoteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "cmt-12345".to_string(),
        url_id: "news/politics/2026-election".to_string(),
        broadcast_id: "broadcast-nytimes-001".to_string(),
        vote_body_params: models::VoteBodyParams { ..Default::default() },
        session_id: Some("sess-9f8e7d".to_string()),
        sso: Some("user-42@example.com".to_string()),
    };
    let response: VoteComment200Response = vote_comment(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---