## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetTrustFactorOptions | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[GetUserTrustFactorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_trust_factor_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getTrustFactor उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (trustOpt, httpResp) = client.getTrustFactor(tenantId = "my-tenant-123", options = GetTrustFactorOptions())
if trustOpt.isSome:
  let trust = trustOpt.get()
  discard trust
[inline-code-end]