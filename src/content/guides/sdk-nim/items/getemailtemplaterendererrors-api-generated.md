## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| skip | float64 | No |  |

## Response

Returns: [`Option[GetEmailTemplateRenderErrors_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors200response.nim)

## Example

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplateRenderErrors(tenantId = "my-tenant-123", id = "welcome-email-456", skip = 0.0)
if response.isSome:
  let errors = response.get()
  echo "Template render errors received:"
  echo errors
else:
  echo "No render errors returned. HTTP status: ", httpResponse.status.code
[inline-code-end]
