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
const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription('tenant_9f8b7a6c', 'sub_3a9d2e7b');
const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription('tenant_9f8b7a6c', 'sub_8b2c4f1d', 'user_47b2f9c1');
[inline-code-end]
