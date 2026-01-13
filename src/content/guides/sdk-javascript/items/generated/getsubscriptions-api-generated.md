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
const tenantId: string = 'tenant_8a7f3b2c';
const userId: string = 'user_4f1c9d6a';
const subscriptions: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
const tenantOnlySubscriptions: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
[inline-code-end]
