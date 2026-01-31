## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[GetModerator_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator200response.nim)

## Example

[inline-code-attrs-start title = 'getModerator Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "")
if response.isSome:
  let moderator = response.get()
  echo "Moderator retrieved: ", moderator
else:
  echo "No moderator found, HTTP status: ", httpResponse.status
[inline-code-end]
