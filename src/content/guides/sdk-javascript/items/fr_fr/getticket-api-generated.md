## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| userId | string | Non |  |

## Réponse

Retourne: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicket200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const ticketId: string = 'TCKT-20250615-42';
const userId: string = 'user_84b2';

const ticketWithUser: GetTicket200Response = await getTicket(tenantId, ticketId, userId);
const ticketWithoutUser: GetTicket200Response = await getTicket(tenantId, ticketId);

console.log(ticketWithUser.id, ticketWithoutUser.id);
[inline-code-end]

---