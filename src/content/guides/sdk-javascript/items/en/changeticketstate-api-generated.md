## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | Yes |  |
| id | string | Yes |  |
| changeTicketStateBody | ChangeTicketStateBody | Yes |  |

## Response

Returns: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ChangeTicketState200Response.ts)

## Example

[inline-code-attrs-start title = 'changeTicketState Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const userId: string = 'agent_204';
const id: string = 'TCKT-2026-047';
const changeTicketStateBody: ChangeTicketStateBody = {
  state: 'resolved',
  comment: 'Fixed in release 1.4.2; verifying with customer before closing',
  notifySubscribers: true, // optional parameter example
};
const result: ChangeTicketState200Response = await changeTicketState(tenantId, userId, id, changeTicketStateBody);
[inline-code-end]
