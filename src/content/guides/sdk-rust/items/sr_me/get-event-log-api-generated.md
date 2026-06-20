req
tenantId
urlId
userIdWS

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| url_id | String | Да |  |
| user_id_ws | String | Да |  |
| start_time | i64 | Да |  |
| end_time | i64 | Не |  |

## Одговор

Враћа: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Пример

[inline-code-attrs-start title = 'get_event_log Пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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