## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| badges_user_id | String | 아니오 |  |
| comment_id | String | 아니오 |  |
| sso | String | 아니오 |  |

## Response

반환: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_manual_badges_response.rs)

## Example

[inline-code-attrs-start title = 'get_manual_badges_for_user 예시'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_badges(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetManualBadgesForUserParams {
        tenant_id: "acme-corp-tenant".to_string(),
        badges_user_id: Some("user-42".to_string()),
        comment_id: Some("comment-987".to_string()),
        sso: Some("sso-abc123".to_string()),
    };
    let _response: GetUserManualBadgesResponse = get_manual_badges_for_user(config, params).await?;
    Ok(())
}
[inline-code-end]