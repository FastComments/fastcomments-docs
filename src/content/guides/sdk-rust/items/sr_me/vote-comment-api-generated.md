## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenant_id | String | Da |  |
| comment_id | String | Da |  |
| url_id | String | Da |  |
| broadcast_id | String | Da |  |
| vote_body_params | models::VoteBodyParams | Da |  |
| session_id | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Primer

[inline-code-attrs-start title = 'vote_comment Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let config = configuration::Configuration::default();

    let vote_body = models::VoteBodyParams {
        vote_type: "upvote".to_string(),
        weight: 1,
    };

    let params = VoteCommentParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "comment-12345".to_string(),
        url_id: "news/article".to_string(),
        broadcast_id: "broadcast-67890".to_string(),
        vote_body_params: vote_body,
        session_id: Some("session-abcde".to_string()),
        sso: None,
    };

    let _response = vote_comment(&config, params).await?;
    Ok(())
}
[inline-code-end]