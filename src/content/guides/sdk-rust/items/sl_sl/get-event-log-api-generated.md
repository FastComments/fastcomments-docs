req
tenantId
urlId
userIdWS

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |
| user_id_ws | String | Da |  |
| start_time | i64 | Da |  |
| end_time | i64 | Ne |  |

## Odgovor

Vrne: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Primer

[inline-code-attrs-start title = 'Primer get_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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