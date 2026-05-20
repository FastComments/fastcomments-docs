## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |

## Response

Returns: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## Example

[inline-code-attrs-start title = 'getSubscriptions Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '9f3b2a1e-4c8d-43b2-9a7d-1e2f3a4b5c6d';
const userId: string = 'user-72f3d8';
const subscriptionsForTenant: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
const subscriptionsForUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
[inline-code-end]
