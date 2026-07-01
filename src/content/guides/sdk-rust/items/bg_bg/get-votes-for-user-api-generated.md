## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| user_id | String | No |  |
| anon_user_id | String | No |  |

## Отговор

Връща: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_votes_for_user_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример за get_votes_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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