## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'deleteEmailTemplate उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.deleteEmailTemplate(tenantId = "my-tenant-123", id = "welcome-email")
if respOpt.isSome:
  let emptyResp = respOpt.get()
  echo "Email template deleted"
else:
  echo "Failed to delete email template"
[inline-code-end]