## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| user_id | String | No |  |
| direction | models::SortDirections | No |  |
| replies_to_user_id | String | No |  |
| page | f64 | No |  |
| includei10n | bool | No |  |
| locale | String | No |  |
| is_crawler | bool | No |  |

## 응답

반환: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## 예제

[inline-code-attrs-start title = 'get_comments_for_user 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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