запит  
tenantId  
urlId  
userIdWS  

## Параметри

| Назва | Тип | Обов’язковий | Опис |
|------|------|--------------|------|
| tenant_id | String | Так |  |
| url_id | String | Так |  |
| user_id_ws | String | Так |  |
| start_time | i64 | Так |  |
| end_time | i64 | Ні |  |

## Відповідь

Returns: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Приклад

[inline-code-attrs-start title = 'Приклад get_global_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetGlobalEventLogParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        user_id_ws: "user-12345".to_string(),
        start_time: 1_680_000_000,
        end_time: Some(1_680_864_000),
    };
    let _response = get_global_event_log(configuration, params).await?;
    Ok(())
}
[inline-code-end]

---