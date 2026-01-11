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
async fn run_checked_comments() -> Result<(), Error> {
    let params: CheckedCommentsForBlockedParams = CheckedCommentsForBlockedParams {
        tenant_id: String::from("acme-news-tenant"),
        comment_ids: String::from("cmt-9876,cmt-1122"),
        sso: Some(String::from("sso-jwt-abc123")),
    };
    let checked: CheckedCommentsForBlocked200Response =
        checked_comments_for_blocked(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
