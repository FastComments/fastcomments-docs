## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |

## Response

Returns: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSubscriptionAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'deleteSubscription Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-84";
const id: string = "sub_92f1b3a7";
const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, id);

const userId: string = "user-2983";
const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, id, userId);
[inline-code-end]
