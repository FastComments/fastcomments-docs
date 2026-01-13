## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetModerators_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators200response.nim)

## Example

[inline-code-attrs-start title = 'getModerators Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let moderators = response.get()
  echo "Moderators fetched successfully"
  echo moderators
[inline-code-end]
