## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenant_id | String | Sim |  |
| id | String | Sim |  |
| user_id | String | Não |  |

## Resposta

Retorna: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_ticket_response.rs)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de get_ticket'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_ticket() -> Result<GetTicketResponse, Error> {
    let params: GetTicketParams = GetTicketParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "TICKET-2026-045".to_string(),
        user_id: Some("user-12345".to_string()),
    };
    let ticket: GetTicketResponse = get_ticket(&configuration, params).await?;
    Ok(ticket)
}
[inline-code-end]

---