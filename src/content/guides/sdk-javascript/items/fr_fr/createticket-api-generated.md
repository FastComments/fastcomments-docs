## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Oui |  |
| createTicketBody | CreateTicketBody | Oui |  |

## Réponse

Renvoie: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-company-001';
const userId: string = 'u_78f4b2';
const createTicketBody: CreateTicketBody = {
  title: 'Unable to access project dashboard',
  description: 'Receiving 403 when accessing /dashboard for project X',
  priority: 'high',
  tags: ['dashboard', 'access'] // champ optionnel démontré
};
const result: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]