## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| id | string | Da |  |
| changeTicketStateBody | ChangeTicketStateBody | Da |  |

## Odgovor

Vraća: [`ChangeTicketStateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse1.ts)

## Primer

[inline-code-attrs-start title = 'Primer changeTicketState'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-97123";
const ticketId: string = "ticket-45001";

const changeTicketStateBody: ChangeTicketStateBody = {
  state: "closed",
  // opciono polje u telu
  comment: "Issue resolved after code fix"
};

const response: ChangeTicketStateResponse1 = await changeTicketState(
  tenantId,
  userId,
  ticketId,
  changeTicketStateBody
);
[inline-code-end]