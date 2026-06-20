## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id | String | Ja |  |
| user_id | String | Nein |  |
| anon_user_id | String | Nein |  |

## Antwort

Gibt zurück: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_votes_for_user Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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