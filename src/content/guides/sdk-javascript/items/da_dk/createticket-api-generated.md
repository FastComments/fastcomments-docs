## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Svar

Returnerer: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises";
const userId: string = "user_12039";
const createTicketBody: CreateTicketBody = {
  subject: "Login failures for multiple users",
  description: "Users report 500 error when authenticating since 2026-06-18 08:00 UTC. Affects web and mobile.",
  priority: "urgent",
  tags: ["authentication", "outage"]
};
const result: CreateTicketResponse = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]

---