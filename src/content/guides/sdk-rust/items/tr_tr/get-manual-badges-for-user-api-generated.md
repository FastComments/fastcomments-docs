## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| badges_user_id | String | Hayır |  |
| comment_id | String | Hayır |  |
| sso | String | Hayır |  |

## Yanıt

Döndürür: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_manual_badges_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_manual_badges_for_user Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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