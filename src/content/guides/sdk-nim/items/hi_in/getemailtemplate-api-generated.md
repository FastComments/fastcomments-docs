## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getEmailTemplate उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email")
if response.isSome:
  let tmpl = response.get()
[inline-code-end]