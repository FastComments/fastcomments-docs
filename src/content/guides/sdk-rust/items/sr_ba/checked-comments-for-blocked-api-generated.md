## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| comment_ids | String | Да |  |
| sso | String | Не |  |

## Одговор

Враћа: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/check_blocked_comments_response.rs)

## Пример

[inline-code-attrs-start title = 'checked_comments_for_blocked Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_checked_comments_for_blocked() -> Result<CheckBlockedCommentsResponse, Error> {
    let params: CheckedCommentsForBlockedParams = CheckedCommentsForBlockedParams {
        tenant_id: "acme-corp-tenant".to_string(),
        comment_ids: "cmt-1023,cmt-2048".to_string(),
        sso: Some("sso:user:john.doe:eyJhbGciOiJIUzI1Ni".to_string()),
    };
    let response: CheckBlockedCommentsResponse = checked_comments_for_blocked(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]