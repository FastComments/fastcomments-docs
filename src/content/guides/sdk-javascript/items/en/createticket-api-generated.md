## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| createTicketBody | CreateTicketBody | Yes |  |

## Response

Returns: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTicket200Response.ts)

## Example

[inline-code-attrs-start title = 'createTicket Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-enterprises-01";
const userId: string = "u_84f3b2";
const createTicketBody: CreateTicketBody = {
  subject: "Unable to access invoices",
  description: "After SSO migration I receive a 403 error when opening the Invoices page.",
  priority: "high",
  tags: ["billing", "sso"],
  assigneeId: "agent_210" // optional parameter demonstrated
};
const response: CreateTicket200Response = await createTicket(tenantId, userId, createTicketBody);
[inline-code-end]
