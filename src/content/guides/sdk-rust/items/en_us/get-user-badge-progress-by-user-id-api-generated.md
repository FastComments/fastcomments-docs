## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| user_id | String | Yes |  |

## Response

Returns: [`ApiGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/api_get_user_badge_progress_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_badge_progress_by_user_id Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let user_id_opt: Option<&str> = Some("user-7823");
    let params: GetUserBadgeProgressByUserIdParams = GetUserBadgeProgressByUserIdParams {
        tenant_id: "acme-corp-tenant".to_string(),
        user_id: user_id_opt.unwrap().to_string(),
    };
    let response: ApiGetUserBadgeProgressResponse =
        get_user_badge_progress_by_user_id(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
