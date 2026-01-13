## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| user_id | String | Не |  |
| anon_user_id | String | Не |  |

## Одговор

Враћа: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_200_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_votes_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_votes() -> Result<GetVotesForUser200Response, Error> {
    let params = GetVotesForUserParams {
        tenant_id: String::from("acme-corp-tenant"),
        url_id: String::from("news/2026/01/12/breaking-tech"),
        user_id: Some(String::from("user-78a3")),
        anon_user_id: Some(String::from("anon-4f2b")),
    };
    let response: GetVotesForUser200Response = get_votes_for_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---