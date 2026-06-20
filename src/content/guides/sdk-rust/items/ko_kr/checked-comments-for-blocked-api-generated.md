## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| comment_ids | String | 예 |  |
| sso | String | 아니오 |  |

## 응답

반환: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/check_blocked_comments_response.rs)

## 예제

[inline-code-attrs-start title = 'checked_comments_for_blocked 예제'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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