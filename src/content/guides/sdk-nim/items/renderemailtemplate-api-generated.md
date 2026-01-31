## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | No |  |
| locale | string | No |  |

## Response

Returns: [`Option[RenderEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template200response.nim)

## Example

[inline-code-attrs-start title = 'renderEmailTemplate Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.renderEmailTemplate(
  tenantId = "my-tenant-123",
  renderEmailTemplateBody = RenderEmailTemplateBody(),
  locale = "en-US"
)

if response.isSome:
  let result = response.get()
  echo result
[inline-code-end]
