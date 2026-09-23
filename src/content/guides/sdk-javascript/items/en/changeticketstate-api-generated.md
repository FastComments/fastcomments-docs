## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| id | string | Yes |  |
| changeTicketStateBody | ChangeTicketStateBody | Yes |  |

## Response

Returns: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketStateResponse.ts)

## Example

[inline-code-attrs-start title = 'changeTicketState Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "acme-corp";
  const userId: string = "john.doe";
  const ticketId: string = "ticket-20230915-001";

  const changeTicketStateBody: ChangeTicketStateBody = {
    // optional field example
    note: "Resolved after investigation"
  };

  const response: ChangeTicketStateResponse = await changeTicketState(
    tenantId,
    userId,
    ticketId,
    changeTicketStateBody
  );

  console.log(response);
})();
[inline-code-end]
