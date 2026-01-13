## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| comment_id | String | Ja |  |
| direction | String | Ja |  |
| user_id | String | Nee |  |
| anon_user_id | String | Nee |  |

## Antwoord

Geeft terug: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/vote_comment_200_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'create_vote Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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