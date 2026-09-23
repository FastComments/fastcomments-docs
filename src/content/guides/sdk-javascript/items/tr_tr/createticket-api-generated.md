## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| userId | string | Evet |  |
| createTicketBody | CreateTicketBody | Evet |  |

## Yanıt

Döndürür: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse.ts)

## Örnek

[inline-code-attrs-start title = 'createTicket Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function submitTicket() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";

  const ticketBody: CreateTicketBody = {
    subject: "Login issues after password reset",
    description: "User reports being unable to log in despite using the new password.",
    priority: "medium",
    // CreateTicketBody içinde isteğe bağlı alan
    tags: ["login", "password-reset"]
  };

  const response: CreateTicketResponse = await createTicket(tenantId, userId, ticketBody);
  console.log(response);
}

submitTicket();
[inline-code-end]