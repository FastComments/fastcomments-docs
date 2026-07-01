req
tenantId
urlId
userIdWS

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenant_id | String | Sim |  |
| url_id | String | Sim |  |
| user_id_ws | String | Sim |  |
| start_time | i64 | Sim |  |
| end_time | i64 | Não |  |

## Resposta

Retorna: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_event_log_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo get_global_event_log'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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