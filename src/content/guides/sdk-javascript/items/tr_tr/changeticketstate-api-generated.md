## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| id | string | Yes |  |
| changeTicketStateBody | ChangeTicketStateBody | Yes |  |

## Yanıt

Döndürür: [`ChangeTicketStateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse1.ts)

## Örnek

[inline-code-attrs-start title = 'changeTicketState Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-97123";
const ticketId: string = "ticket-45001";

const changeTicketStateBody: ChangeTicketStateBody = {
  state: "closed",
  // body içinde isteğe bağlı alan
  comment: "Issue resolved after code fix"
};

const response: ChangeTicketStateResponse1 = await changeTicketState(
  tenantId,
  userId,
  ticketId,
  changeTicketStateBody
);
[inline-code-end]