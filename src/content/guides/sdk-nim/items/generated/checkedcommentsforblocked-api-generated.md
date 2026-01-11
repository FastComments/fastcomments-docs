## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentIds | string | No |  |
| sso | string | No |  |

## Response

Returns: [`Option[CheckedCommentsForBlocked_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_checked_comments_for_blocked200response.nim)

## Example

[inline-code-attrs-start title = 'checkedCommentsForBlocked Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.checkedCommentsForBlocked(tenantId = "my-tenant-123",
  commentIds = "comment-1122,comment-3344",
  sso = "")

if response.isSome:
  let checked = response.get()
  echo "Checked comments received: ", $checked
else:
  echo "No checked comments returned, HTTP status: ", $httpResponse.status
[inline-code-end]
