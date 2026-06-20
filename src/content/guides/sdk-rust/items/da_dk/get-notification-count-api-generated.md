## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| user_id | String | Nej |  |
| url_id | String | Nej |  |
| from_comment_id | String | Nej |  |
| viewed | bool | Nej |  |

## Svar

Returnerer: [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_notification_count_response.rs)

## Eksempel

[inline-code-attrs-start title = 'get_notification_count Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetNotificationCountParams = GetNotificationCountParams {
    tenant_id: "acme-corp-tenant".to_string(),
    user_id: Some("user-123".to_string()),
    url_id: Some("news/article/2026/06/19".to_string()),
    from_comment_id: Some("cmt-98765".to_string()),
    viewed: Some(false),
};
let notification_count: GetNotificationCountResponse = get_notification_count(configuration, params).await?;
[inline-code-end]

---