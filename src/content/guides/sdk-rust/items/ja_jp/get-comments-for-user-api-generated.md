## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| user_id | String | いいえ |  |
| direction | models::SortDirections | いいえ |  |
| replies_to_user_id | String | いいえ |  |
| page | f64 | いいえ |  |
| includei10n | bool | いいえ |  |
| locale | String | いいえ |  |
| is_crawler | bool | いいえ |  |

## レスポンス

返却: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## 例

[inline-code-attrs-start title = 'get_comments_for_user の例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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