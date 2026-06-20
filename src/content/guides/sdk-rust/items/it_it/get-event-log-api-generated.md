req
tenantId
urlId
userIdWS

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Sì |  |
| url_id | String | Sì |  |
| user_id_ws | String | Sì |  |
| start_time | i64 | Sì |  |
| end_time | i64 | No |  |

## Risposta

Restituisce: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio di get_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_event_log() -> Result<(), Error> {
    let params = GetEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article/2024-product-launch".to_string(),
        user_id_ws: "user_98765_ws".to_string(),
        start_time: 1710700800i64,
        end_time: Some(1710787200i64),
    };
    let response: GetEventLogResponse = get_event_log(&configuration, params).await?;
    println!("{:#?}", response);
    Ok(())
}
[inline-code-end]