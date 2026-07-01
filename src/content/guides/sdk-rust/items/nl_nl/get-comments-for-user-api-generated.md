## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| user_id | String | Nee |  |
| direction | models::SortDirections | Nee |  |
| replies_to_user_id | String | Nee |  |
| page | f64 | Nee |  |
| includei10n | bool | Nee |  |
| locale | String | Nee |  |
| is_crawler | bool | Nee |  |

## Respons

Retourneert: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## Voorbeeld

[inline-code-attrs-start title = 'get_comments_for_user Voorbeeld'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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