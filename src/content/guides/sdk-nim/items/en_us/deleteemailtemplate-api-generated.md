## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'deleteEmailTemplate Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplate(
  tenantId = "my-tenant-123",
  id = "welcome-email-template-001"
)

if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
  echo "Email template deleted successfully"
else:
  echo "No response body"
[inline-code-end]
