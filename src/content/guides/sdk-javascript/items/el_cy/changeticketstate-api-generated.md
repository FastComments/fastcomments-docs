## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| userId | string | Ναι |  |
| id | string | Ναι |  |
| changeTicketStateBody | ChangeTicketStateBody | Ναι |  |

## Απάντηση

Επιστρέφει: [`ChangeTicketStateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse1.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα changeTicketState'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-97123";
const ticketId: string = "ticket-45001";

const changeTicketStateBody: ChangeTicketStateBody = {
  state: "closed",
  // προαιρετικό πεδίο στο σώμα
  comment: "Issue resolved after code fix"
};

const response: ChangeTicketStateResponse1 = await changeTicketState(
  tenantId,
  userId,
  ticketId,
  changeTicketStateBody
);
[inline-code-end]