## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|--------|------|-----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Respuesta

Devuelve: [`CreateTicketResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const userId: string = "user_98765";

const ticketBody: CreateTicketBody = {
  subject: "Issue with payment processing"
  // description?: string es opcional y se omite
};

const response: CreateTicketResponse1 = await createTicket(tenantId, userId, ticketBody);
// Ejemplo de uso de un campo opcional de la respuesta
// console.log(response.ticket?.id);
[inline-code-end]

---