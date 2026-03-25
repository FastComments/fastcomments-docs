## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Sì |  |
| url_id | String | Sì |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Risposta

Restituisce: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_200_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_votes_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_votes_for_user() -> Result<(), Error> {
    let params: GetVotesForUserParams = GetVotesForUserParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/article-2026-03-fastcomments-launch"),
        user_id: Some(String::from("user_12345")),
        anon_user_id: Some(String::from("anon_9f2e7b")),
    };
    let votes: GetVotesForUser200Response = get_votes_for_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---