## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| sure | string = "" | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'deleteTenant उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.deleteTenant(tenantId = "my-tenant-123", id = "tenant-to-delete", sure = "yes")
if respOpt.isSome:
  let emptyResp = respOpt.get()
[inline-code-end]