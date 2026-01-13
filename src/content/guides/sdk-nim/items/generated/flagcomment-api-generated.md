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

[inline-code-attrs-start title = 'flagComment Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(tenantId = "my-tenant-123", id = "comment-9876", userId = "", anonUserId = "anon-4f2b9c")

if response.isSome:
  let flagResult = response.get()
  echo "Flag succeeded: ", $flagResult
else:
  echo "Flag failed, HTTP response status: ", $httpResponse.status
[inline-code-end]
