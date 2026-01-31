## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| comment_ids | String | Yes |  |
| sso | String | No |  |

## Response

Returns: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/checked_comments_for_blocked_200_response.rs)

## Example

[inline-code-attrs-start title = 'checked_comments_for_blocked Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CheckedCommentsForBlockedParams = CheckedCommentsForBlockedParams {
        tenant_id: String::from("acme-corp-tenant"),
        comment_ids: String::from("comment-1024,comment-2048,comment-4096"),
        sso: Some(String::from("sso_jwt:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9")),
    };
    let response: CheckedCommentsForBlocked200Response =
        checked_comments_for_blocked(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
