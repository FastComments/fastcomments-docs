## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| user_id | String | Ne |  |
| direction | models::SortDirections | Ne |  |
| replies_to_user_id | String | Ne |  |
| page | f64 | Ne |  |
| includei10n | bool | Ne |  |
| locale | String | Ne |  |
| is_crawler | bool | Ne |  |

## Odgovor

Vrati: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_comments_for_user'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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