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
  const tenantId: string = "a3f1c9e2-7b4d-4c2f-8a9e-0b1c2d3e4f5a";
  const subscriptionId: string = "4b6d2a3e-9f1b-45c7-8d2e-6a7b8c9d0e1f";
  const userId: string = "f47ac10b-58cc-4372-a567-0e02b2c3d479";
  const resultWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId);
  const resultWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId, userId);
})();
[inline-code-end]
