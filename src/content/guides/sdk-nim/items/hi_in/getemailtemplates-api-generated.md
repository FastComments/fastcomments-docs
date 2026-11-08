## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| skip | float64 | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetEmailTemplatesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_templates_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getEmailTemplates उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (emailTemplatesOpt, httpResp) = client.getEmailTemplates(tenantId = "my-tenant-123", skip = 0.0)
if emailTemplatesOpt.isSome:
  let templates = emailTemplatesOpt.get()
  echo templates
[inline-code-end]