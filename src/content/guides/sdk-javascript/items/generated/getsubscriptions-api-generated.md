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
(async () => {
  const tenantId: string = "tenant_acme_01";
  const userId: string = "user_jdoe_42";
  const allSubscriptions: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
  const userSubscriptions: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
  console.log(allSubscriptions, userSubscriptions);
})();
[inline-code-end]
