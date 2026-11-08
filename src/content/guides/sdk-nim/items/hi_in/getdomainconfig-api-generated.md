## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| domain | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getDomainConfig उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (configOpt, httpResp) = client.getDomainConfig(tenantId = "my-tenant-123", domain = "news.example.com")
if configOpt.isSome:
  let cfg = configOpt.get()
  discard cfg
[inline-code-end]