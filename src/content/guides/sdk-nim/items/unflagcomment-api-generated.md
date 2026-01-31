## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## Example

[inline-code-attrs-start title = 'unFlagComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unFlagComment(tenantId = "my-tenant-123",
  id = "comment-7f3a2b",
  userId = "",
  anonUserId = "")

if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]
