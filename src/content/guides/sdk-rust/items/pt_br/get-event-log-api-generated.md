---
req
tenantId
urlId
userIdWS

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| user_id_ws | String | Sim |  |
| start_time | i64 | Sim |  |
| end_time | i64 | Não |  |

## Resposta

Retorna: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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