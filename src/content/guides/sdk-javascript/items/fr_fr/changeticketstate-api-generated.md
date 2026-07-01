## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| id | string | Yes |  |
| changeTicketStateBody | ChangeTicketStateBody | Yes |  |

## Réponse

Renvoie : [`ChangeTicketStateResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse1.ts)

## Exemple

[inline-code-attrs-start title = 'changeTicketState Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-97123";
const ticketId: string = "ticket-45001";

const changeTicketStateBody: ChangeTicketStateBody = {
  state: "closed",
  // champ facultatif dans le corps
  comment: "Issue resolved after code fix"
};

const response: ChangeTicketStateResponse1 = await changeTicketState(
  tenantId,
  userId,
  ticketId,
  changeTicketStateBody
);
[inline-code-end]

---