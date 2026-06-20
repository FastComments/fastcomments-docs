## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| user_id | String | 否 |  |
| direction | models::SortDirections | 否 |  |
| replies_to_user_id | String | 否 |  |
| page | f64 | 否 |  |
| includei10n | bool | 否 |  |
| locale | String | 否 |  |
| is_crawler | bool | 否 |  |

## 回應

回傳: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_comments_for_user_response.rs)

## 範例

[inline-code-attrs-start title = 'get_comments_for_user 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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