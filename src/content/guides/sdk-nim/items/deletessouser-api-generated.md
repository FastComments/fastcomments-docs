## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| deleteComments | bool | No |  |
| commentDeleteMode | string | No |  |

## Response

Returns: [`Option[DeleteSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_delete_sso_user_api_response.nim)

## Example

[inline-code-attrs-start title = 'deleteSSOUser Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteSSOUser(
  tenantId = "my-tenant-123",
  id = "user-987654",
  deleteComments = true,
  commentDeleteMode = "anonymize"
)

if response.isSome:
  let deleteResp = response.get()
  echo "DeleteSSOUser response:", deleteResp
else:
  echo "No response body, HTTP status:", httpResponse.status
[inline-code-end]
