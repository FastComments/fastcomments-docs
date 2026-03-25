## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| id | string | Oui |  |
| userId | string | Non |  |

## Réponse

Retourne: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTicket200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const ticketId: string = 'tkt-20260325-42';
const userId: string = 'user-8452';

const ticketResponseWithUser: GetTicket200Response = await getTicket(tenantId, ticketId, userId);
const ticketResponseWithoutUser: GetTicket200Response = await getTicket(tenantId, ticketId);
[inline-code-end]

---