## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'deleteEmailTemplate Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email-2026")
if response.isSome:
  let deleted = response.get()
  echo "Template deleted successfully"
else:
  echo "No response body returned"
[inline-code-end]
