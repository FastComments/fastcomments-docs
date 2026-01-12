## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id_ws | String | Yes |  |
| user_ids | String | Yes |  |

## Response

Returns: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_presence_statuses_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_presence_statuses Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let user_ids: Option<String> = Some("user-123,user-456".to_string());
    let params: GetUserPresenceStatusesParams = GetUserPresenceStatusesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id_ws: "news/article".to_string(),
        user_ids: user_ids.unwrap_or_else(|| "user-123".to_string()),
    };
    let presence: GetUserPresenceStatuses200Response = get_user_presence_statuses(&configuration, params).await?;
    Ok(())
}
[inline-code-end]
