## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| user_id | String | Não |  |
| state | f64 | Não |  |
| skip | f64 | Não |  |
| limit | f64 | Não |  |

## Resposta

Retorna: [`GetTicketsResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_tickets_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_tickets'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example_get_tickets() -> Result<(), Error> {
    let params: GetTicketsParams = GetTicketsParams {
        tenant_id: String::from("acme-corp-tenant"),
        user_id: Some(String::from("journalist-42")),
        state: Some(1.0),
        skip: Some(0.0),
        limit: Some(50.0),
    };
    let tickets: GetTicketsResponse = get_tickets(&configuration, params).await?;
    Ok(())
}
[inline-code-end]