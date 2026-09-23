## Parameters

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Response

Повертає: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse.ts)

## Example

[inline-code-attrs-start title = 'Приклад createTicket'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function submitTicket() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";

  const ticketBody: CreateTicketBody = {
    subject: "Login issues after password reset",
    description: "User reports being unable to log in despite using the new password.",
    priority: "medium",
    // необов’язкове поле в CreateTicketBody
    tags: ["login", "password-reset"]
  };

  const response: CreateTicketResponse = await createTicket(tenantId, userId, ticketBody);
  console.log(response);
}

submitTicket();
[inline-code-end]

---