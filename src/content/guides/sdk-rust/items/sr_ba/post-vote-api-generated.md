## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| comment_id | String | Da |  |
| direction | String | Ne |  |
| sso | String | Ne |  |

## Odgovor

Vraća: [`VoteResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_response.rs)

## Primjer

[inline-code-attrs-start title = 'post_vote Primjer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn submit_vote() -> Result<VoteResponse, Error> {
    let params: PostVoteParams = PostVoteParams {
        comment_id: String::from("news/article-1234/comment-5678"),
        direction: Some(String::from("up")),
        sso: Some(String::from("acme-corp-sso-token-abc123")),
    };
    let vote_response: VoteResponse = post_vote(&configuration, params).await?;
    Ok(vote_response)
}
[inline-code-end]

---