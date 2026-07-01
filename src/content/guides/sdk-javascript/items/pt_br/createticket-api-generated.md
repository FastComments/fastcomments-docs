## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| userId | string | Sim |  |
| createTicketBody | CreateTicketBody | Sim |  |

## Resposta

Retorna: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string é opcional e omitido
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Exemplo de uso de um campo opcional da resposta
// console.log(response.ticket?.id);
[inline-code-end]