## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| user_id | String | Ні |  |
| direction | models::SortDirections | Ні |  |
| replies_to_user_id | String | Ні |  |
| page | f64 | Ні |  |
| includei10n | bool | Ні |  |
| locale | String | Ні |  |
| is_crawler | bool | Ні |  |

## Відповідь

Повертає: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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