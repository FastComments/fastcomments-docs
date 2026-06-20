## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| badges_user_id | String | 否 |  |
| comment_id | String | 否 |  |
| sso | String | 否 |  |

## 回應

回傳：[`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_manual_badges_response.rs)

## 範例

[inline-code-attrs-start title = 'get_manual_badges_for_user 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_manual_badges() -> Result<GetUserManualBadgesResponse, Error> {
    let params: GetManualBadgesForUserParams = GetManualBadgesForUserParams {
        badges_user_id: Some(String::from("acme-user-42")),
        comment_id: Some(String::from("news/article-5678")),
        sso: Some(String::from("sso-token-abc123")),
    };
    let response: GetUserManualBadgesResponse = get_manual_badges_for_user(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---