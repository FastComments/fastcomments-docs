## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
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