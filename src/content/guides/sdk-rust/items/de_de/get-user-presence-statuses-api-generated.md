## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| url_id_ws | String | Ja |  |
| user_ids | String | Ja |  |

## Antwort

Gibt zurück: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_presence_statuses_response.rs)

## Beispiel

[inline-code-attrs-start title = 'get_user_presence_statuses Beispiel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let cfg: &configuration::Configuration = &configuration;
let params: GetUserPresenceStatusesParams = GetUserPresenceStatusesParams {
    tenant_id: "acme-corp-tenant".to_string(),
    url_id_ws: "news/article".to_string(),
    user_ids: "user-123,user-456".to_string(),
    include_offline: Some(false),
};
let response: GetUserPresenceStatusesResponse = get_user_presence_statuses(cfg, params).await?;
[inline-code-end]

---