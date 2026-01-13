## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| comment_id | String | Oui |  |
| direction | String | Oui |  |
| user_id | String | Non |  |
| anon_user_id | String | Non |  |

## Réponse

Renvoie: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de create_vote'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_vote() -> Result<VoteComment200Response, Error> {
    let params: CreateVoteParams = CreateVoteParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_id: "news/2026/01/12/local-election-12345".to_string(),
        direction: "up".to_string(),
        user_id: Some("user_9876".to_string()),
        anon_user_id: None,
    };
    let response: VoteComment200Response = create_vote(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---