## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| value | String | No |  |
| sso | String | No |  |

## Response

Returns: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/moderation_user_search_response.rs)

## Example

[inline-code-attrs-start title = 'get_search_users Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetSearchUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        value: Some("john.doe".to_string()),
        sso: Some("sso-provider".to_string()),
    };
    let _response: ModerationUserSearchResponse = get_search_users(configuration, params).await?;
    Ok(())
}
[inline-code-end]
