## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| skip | float64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getEmailTemplateRenderErrors(
  tenantId = "my-tenant-123",
  id = "welcome-template",
  skip = 0.0
)

if optResp.isSome:
  let resp = optResp.get()
  # resp का आवश्यकतानुसार उपयोग करें
else:
  # गुम प्रतिक्रिया को संभालें
  discard
[inline-code-end]