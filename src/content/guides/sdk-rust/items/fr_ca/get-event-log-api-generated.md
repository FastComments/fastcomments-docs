req
tenantId
urlId
userIdWS

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| url_id | String | Oui |  |
| user_id_ws | String | Oui |  |
| start_time | i64 | Oui |  |
| end_time | i64 | Non |  |

## Réponse

Retourne : [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Exemple

[inline-code-attrs-start title = 'get_event_log Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_event_log(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        user_id_ws: "user-12345".to_string(),
        start_time: 1_640_995_200,
        end_time: Some(1_640_995_300),
    };
    let _response: GetEventLogResponse = get_event_log(configuration, params).await?;
    Ok(())
}
[inline-code-end]