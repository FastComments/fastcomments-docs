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
const tenantId: string = "contoso-9a1b2c";
const userId: string = "u-482f6";
const subscriptions: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
const userSubscriptions: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
[inline-code-end]
