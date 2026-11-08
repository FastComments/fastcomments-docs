## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| user_id | String | नहीं |  |
| direction | models::SortDirections | नहीं |  |
| replies_to_user_id | String | नहीं |  |
| page | f64 | नहीं |  |
| includei10n | bool | नहीं |  |
| locale | String | नहीं |  |
| is_crawler | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_comments_for_user उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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