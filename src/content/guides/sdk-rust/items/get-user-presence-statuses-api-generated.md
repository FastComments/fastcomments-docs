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
async fn run_example() -> Result<(), Error> {
    let user_ids_option: Option<String> = Some("user_123,user_456".to_string());
    let params: GetUserPresenceStatusesParams = GetUserPresenceStatusesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id_ws: "news/article/2025/01/12/breaking".to_string(),
        user_ids: user_ids_option.unwrap(),
    };
    let statuses: GetUserPresenceStatuses200Response = get_user_presence_statuses(&configuration, params).await?;
    let _ = statuses;
    Ok(())
}
[inline-code-end]
