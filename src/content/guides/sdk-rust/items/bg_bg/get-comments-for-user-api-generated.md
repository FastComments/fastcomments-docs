## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| user_id | String | Не |  |
| direction | models::SortDirections | Не |  |
| replies_to_user_id | String | Не |  |
| page | f64 | Не |  |
| includei10n | bool | Не |  |
| locale | String | Не |  |
| is_crawler | bool | Не |  |

## Отговор

Връща: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## Пример

[inline-code-attrs-start title = 'get_comments_for_user Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_user_comments() -> Result<(), Error> {
    let params = GetCommentsForUserParams {
        user_id: Some("user-42".to_string()),
        direction: Some(models::SortDirections::Desc),
        replies_to_user_id: Some("reply-to-42".to_string()),
        page: Some(1.0),
        includei10n: Some(true),
        locale: Some("en-US".to_string()),
        is_crawler: Some(false),
    };
    let _response = get_comments_for_user(&configuration, params).await?;
    Ok(())
}
[inline-code-end]