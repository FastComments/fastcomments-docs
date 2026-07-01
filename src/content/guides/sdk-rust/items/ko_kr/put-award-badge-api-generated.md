## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| badge_id | String | 예 |  |
| user_id | String | 아니오 |  |
| comment_id | String | 아니오 |  |
| broadcast_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/award_user_badge_response.rs)

## 예제

[inline-code-attrs-start title = 'put_award_badge 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = PutAwardBadgeParams {
        tenant_id: "acme-corp".to_string(),
        badge_id: "top-contributor".to_string(),
        user_id: Some("user-42".to_string()),
        comment_id: Some("comment-99".to_string()),
        broadcast_id: None,
        sso: Some("sso-abc123".to_string()),
    };
    let _response = put_award_badge(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---