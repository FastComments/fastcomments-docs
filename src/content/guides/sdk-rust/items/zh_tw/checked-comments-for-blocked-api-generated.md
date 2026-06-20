---
## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| comment_ids | String | 是 |  |
| sso | String | 否 |  |

## 回應

回傳: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/check_blocked_comments_response.rs)

## 範例

[inline-code-attrs-start title = 'checked_comments_for_blocked 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---