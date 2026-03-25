## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenantId | string | Sim |  |
| userId | string | Sim |  |
| id | string | Sim |  |
| changeTicketStateBody | ChangeTicketStateBody | Sim |  |

## Resposta

Retorna: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de changeTicketState'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_82f9b3';
const userId: string = 'user_9472';
const id: string = 'ticket_550e8400-e29b-41d4-a716-446655440000';
const changeTicketStateBody: ChangeTicketStateBody = {
  state: 'resolved',
  comment: 'Confirmed fix deployed to production; closing ticket.',
  notifySubscribers: true
};
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---