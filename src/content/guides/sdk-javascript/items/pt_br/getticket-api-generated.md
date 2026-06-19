## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| id | string | Sim |  |
| userId | string | Não |  |

## Resposta

Retorna: [`GetTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicketResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const ticketResponse: GetTicketResponse = await getTicket("fc_tenant_1a2b3c", "TK-20260619-0042", "user_2481");
const ticketResponseNoUser: GetTicketResponse = await getTicket("fc_tenant_1a2b3c", "TK-20260619-0043");
[inline-code-end]

---