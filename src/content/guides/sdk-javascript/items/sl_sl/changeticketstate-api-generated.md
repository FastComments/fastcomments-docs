## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| id | string | Da |  |
| changeTicketStateBody | ChangeTicketStateBody | Da |  |

## Odgovor

Vrne: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse.ts)

## Primer

[inline-code-attrs-start title = 'changeTicketState Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe";
  const ticketId: string = "ticket-20230915-001";

  const changeTicketStateBody: ChangeTicketStateBody = {
    // primer neobveznega polja
    note: "Resolved after investigation"
  };

  const response: ChangeTicketStateResponse = await changeTicketState(
    tenantId,
    userId,
    ticketId,
    changeTicketStateBody
  );

  console.log(response);
})();
[inline-code-end]