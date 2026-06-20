## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| user_id | String | Non |  |
| anon_user_id | String | Non |  |

## Réponse

Renvoie : [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_response.rs)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_votes_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes_for_user() -> Result<(), Error> {
    let params: GetVotesForUserParams = GetVotesForUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/2026/06/15/market-update".to_string(),
        user_id: Some("user_98765".to_string()),
        anon_user_id: Some("anon-4f3b2a".to_string()),
    };
    let response: GetVotesForUserResponse = get_votes_for_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---