## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | seq[string] | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[GetUserReactsPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_reacts_public200response.nim)

## Example

[inline-code-attrs-start title = 'getUserReactsPublic Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserReactsPublic(tenantId = "my-tenant-123", postIds = @["post-987", "post-654"], sso = "sso-token-abc123")
if response.isSome:
  let reacts = response.get()
  echo "Received reacts:", repr(reacts)
else:
  echo "No reacts returned, HTTP status:", $httpResponse.status
[inline-code-end]
