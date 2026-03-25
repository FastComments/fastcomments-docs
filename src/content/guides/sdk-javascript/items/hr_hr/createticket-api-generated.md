## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |
| createTicketBody | CreateTicketBody | Da |  |

## Odgovor

Vraća: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-001";
const userId: string = "user_72b9f4";
const createTicketBody: CreateTicketBody = {
  subject: "Subscription renewal failed for card on file",
  description: "Customer's card was declined by the payment processor during automatic renewal. Transaction ID: txn_9a8b7c. Please review gateway logs and retry.",
  priority: "high", // primjer neobaveznog polja
  contactEmail: "billing@acme-corp.com", // primjer neobaveznih kontakt informacija
  relatedUrl: "https://acme-corp.com/account/billing"
};
const ticketResponse: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]

---