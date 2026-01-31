## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| errorId | string | No |  |

## Response

Returns: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Example

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplateRenderError(tenantId = "my-tenant-123", id = "welcome-email-template", errorId = "render-error-001")
if response.isSome:
  let deleted = response.get()
  echo "Render error deleted: ", $deleted
else:
  echo "No response body received"
[inline-code-end]
