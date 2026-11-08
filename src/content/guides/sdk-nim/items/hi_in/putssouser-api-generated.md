## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | नहीं |  |
| updateAPISSOUserData | UpdateAPISSOUserData | नहीं |  |
| updateComments | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[PutSSOUserAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_sso_user_api_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'putSSOUser उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.putSSOUser(
  tenantId = "my-tenant-123",
  id = "user-456",
  updateAPISSOUserData = default(UpdateAPISSOUserData),
  updateComments = false)

if apiRespOpt.isSome:
  let apiResp = apiRespOpt.get()
  echo apiResp
[inline-code-end]