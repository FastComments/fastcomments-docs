## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| id | string | Da |  |
| changeTicketStateBody | ChangeTicketStateBody | Da |  |

## Odgovor

Vraća: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer changeTicketState'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f3b2c9a";
const userId: string = "user_5a1d9fb2";
const id: string = "ticket_3e8a1b6f";
const changeTicketStateBody: ChangeTicketStateBody = {
  state: "closed",
  reason: "Fixed in backend release 2.4.1",
  notifyUsers: true,
  metadata: { resolutionOwner: "agent_12", priority: "high" } // demonstrirana neobavezna polja
};
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]

---