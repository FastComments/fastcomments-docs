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
(async () => {
  const tenantId: string = "acme-enterprises-123";
  const subscriptionId: string = "sub_8f2b4c9d";
  const userId: string = "user_42b7a";
  const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId, userId);
  const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId);
})();
[inline-code-end]
