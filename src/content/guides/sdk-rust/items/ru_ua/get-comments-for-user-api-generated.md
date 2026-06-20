## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| user_id | String | Нет |  |
| direction | models::SortDirections | Нет |  |
| replies_to_user_id | String | Нет |  |
| page | f64 | Нет |  |
| includei10n | bool | Нет |  |
| locale | String | Нет |  |
| is_crawler | bool | Нет |  |

## Ответ

Возвращает: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример get_comments_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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