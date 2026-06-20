## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| badges_user_id | String | 아니요 |  |
| comment_id | String | 아니요 |  |
| sso | String | 아니요 |  |

## 응답

반환: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_manual_badges_response.rs)

## 예제

[inline-code-attrs-start title = 'get_manual_badges_for_user 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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