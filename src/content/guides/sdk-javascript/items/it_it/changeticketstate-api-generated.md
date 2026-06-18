## Parametri

| Name | Type | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| userId | string | Sì |  |
| id | string | Sì |  |
| changeTicketStateBody | ChangeTicketStateBody | Sì |  |

## Risposta

Restituisce: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di changeTicketState'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8a7d3f4b';
const userId: string = 'user_5d1a9b2c';
const id: string = 'ticket_1024';
const changeTicketStateBody: ChangeTicketStateBody = {
  state: 'closed',
  notifyParticipants: true, // esempio di parametro opzionale
  comment: 'Resolved by support — follow-up not required.'
};
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---