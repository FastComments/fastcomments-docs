req
tenantId
urlId
userIdWS

## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenant_id | String | Tak |  |
| url_id | String | Tak |  |
| user_id_ws | String | Tak |  |
| start_time | i64 | Tak |  |
| end_time | i64 | Nie |  |

## Odpowiedź

Zwraca: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Przykład

[inline-code-attrs-start title = 'Przykład get_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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

---