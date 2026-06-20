## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| user_id | String | Nej |  |
| direction | models::SortDirections | Nej |  |
| replies_to_user_id | String | Nej |  |
| page | f64 | Nej |  |
| includei10n | bool | Nej |  |
| locale | String | Nej |  |
| is_crawler | bool | Nej |  |

## Svar

Returnerer: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_comments_for_user Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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