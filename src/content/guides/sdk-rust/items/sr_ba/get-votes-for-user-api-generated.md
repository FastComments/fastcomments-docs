## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| user_id | String | Не |  |
| anon_user_id | String | Не |  |

## Одговор

Враћа: [`GetVotesForUser200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_200_response.rs)

## Пример

[inline-code-attrs-start title = 'get_votes_for_user Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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