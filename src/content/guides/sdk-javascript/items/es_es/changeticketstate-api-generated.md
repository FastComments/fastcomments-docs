## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|--------|------|-------------|-------------|
| tenantId | string | Sí |  |
| userId | string | Sí |  |
| id | string | Sí |  |
| changeTicketStateBody | ChangeTicketStateBody | Sí |  |

## Respuesta

Devuelve: [`ChangeTicketStateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse1.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de changeTicketState'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-97123";
const ticketId: string = "ticket-45001";

const changeTicketStateBody: ChangeTicketStateBody = {
  state: "closed",
  // campo opcional en el cuerpo
  comment: "Issue resolved after code fix"
};

const response: ChangeTicketStateResponse1 = await changeTicketState(
  tenantId,
  userId,
  ticketId,
  changeTicketStateBody
);
[inline-code-end]