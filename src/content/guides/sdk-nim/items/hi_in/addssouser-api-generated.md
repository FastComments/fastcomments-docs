## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createAPISSOUserData | CreateAPISSOUserData | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[AddSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_sso_user_api_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'addSSOUser उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.addSSOUser(tenantId = "my-tenant-123", createAPISSOUserData = CreateAPISSOUserData())
if optResp.isSome:
  let userResp = optResp.get()
[inline-code-end]