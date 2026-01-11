## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string | No |  |

## Response

Returns: [`Option[DeleteSubscriptionAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_subscription_api_response.nim)

## Example

[inline-code-attrs-start title = 'deleteSubscription Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteSubscription(tenantId = "my-tenant-123", id = "", userId = "")
if response.isSome:
  let deleteResp = response.get()
  echo "Delete subscription response received for tenant: my-tenant-123"
[inline-code-end]
