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
  const tenantId: string = "fc_tenant_9a7f";
  const resultNoUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
  const userId: string = "user_7c9b_jdoe";
  const resultWithUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
  console.log(resultNoUser, resultWithUser);
})();
[inline-code-end]
