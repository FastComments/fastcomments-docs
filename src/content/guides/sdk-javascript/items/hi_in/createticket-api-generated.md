## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| userId | string | हाँ |  |
| createTicketBody | CreateTicketBody | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicketResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createTicket उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function submitTicket() {
  const tenantId: string = "acme-corp";
  const userId: string = "user-9876";

  const ticketBody: CreateTicketBody = {
    subject: "Login issues after password reset",
    description: "User reports being unable to log in despite using the new password.",
    priority: "medium",
    // CreateTicketBody में वैकल्पिक फ़ील्ड
    tags: ["login", "password-reset"]
  };

  const response: CreateTicketResponse = await createTicket(tenantId, userId, ticketBody);
  console.log(response);
}

submitTicket();
[inline-code-end]