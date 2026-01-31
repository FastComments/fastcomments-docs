## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| page_size | i32 | No |  |
| after_id | String | No |  |
| include_context | bool | No |  |
| after_created_at | i64 | No |  |
| unread_only | bool | No |  |
| dm_only | bool | No |  |
| no_dm | bool | No |  |
| include_translations | bool | No |  |
| sso | String | No |  |

## Response

Returns: [`GetUserNotifications200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_notifications_200_response.rs)

## Example

[inline-code-attrs-start title = 'get_user_notifications Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetUserNotificationsParams = GetUserNotificationsParams {
    tenant_id: String::from("acme-news-tenant"),
    page_size: Some(25),
    after_id: Some(String::from("notif_abc123")),
    include_context: Some(true),
    after_created_at: Some(1672531200i64),
    unread_only: Some(true),
    dm_only: Some(false),
    no_dm: Some(false),
    include_translations: Some(true),
    sso: Some(String::from("user-123-sso")),
};

let notifications: GetUserNotifications200Response = get_user_notifications(&configuration, params).await?;
[inline-code-end]
