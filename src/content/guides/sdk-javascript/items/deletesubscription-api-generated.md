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
const tenantId: string = 'tenant_acme_9f3b';
const subscriptionIdNoUser: string = 'sub_4b7e9f';
const subscriptionIdWithUser: string = 'sub_7c2d1a';
const userId: string = 'user_88a2';
const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionIdNoUser);
const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionIdWithUser, userId);
[inline-code-end]
