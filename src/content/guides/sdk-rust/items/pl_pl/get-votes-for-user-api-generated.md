## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| user_id | String | Nie |  |
| anon_user_id | String | Nie |  |

## Odpowiedź

Zwraca: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_response.rs)

## Przykład

[inline-code-attrs-start title = 'get_votes_for_user Przykład'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetVotesForUserParams {
        tenant_id: "acme-corp".to_string(),
        url_id: "news/2023/09/awesome-article".to_string(),
        user_id: Some("user-12345".to_string()),
        anon_user_id: None,
    };
    let _response = get_votes_for_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---