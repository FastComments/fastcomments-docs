## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| user_id | String | Не |  |
| direction | models::SortDirections | Не |  |
| replies_to_user_id | String | Не |  |
| page | f64 | Не |  |
| includei10n | bool | Не |  |
| locale | String | Не |  |
| is_crawler | bool | Не |  |

## Одговор

Враћа: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## Пример

[inline-code-attrs-start title = 'get_comments_for_user Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params: GetCommentsForUserParams = GetCommentsForUserParams {
        user_id: Some("alice@acme-corp".to_string()),
        direction: Some(models::SortDirections::Descending),
        replies_to_user_id: Some("editor-202".to_string()),
        page: Some(1.0),
        includei10n: Some(true),
        locale: Some("en-US".to_string()),
        is_crawler: Some(false),
    };
    let response: GetCommentsForUserResponse = get_comments_for_user(configuration, params).await?;
    let _ = response;
    Ok(())
}
[inline-code-end]

---