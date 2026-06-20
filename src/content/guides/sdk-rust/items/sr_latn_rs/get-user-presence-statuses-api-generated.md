## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id_ws | String | Da |  |
| user_ids | String | Da |  |

## Odgovor

Vraća: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_user_presence_statuses_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_user_presence_statuses'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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