## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Oui |  |
| createTicketBody | CreateTicketBody | Oui |  |

## Réponse

Renvoie : [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string est optionnel et omis
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Exemple d'utilisation d'un champ optionnel de la réponse
// console.log(response.ticket?.id);
[inline-code-end]