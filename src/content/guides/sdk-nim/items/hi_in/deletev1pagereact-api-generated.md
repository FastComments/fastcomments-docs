## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## उदाहरण

[inline-code-attrs-start title = 'deleteV1PageReact उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (reactOpt, httpResp) = client.deleteV1PageReact(tenantId = "my-tenant-123", urlId = "news/article-title")
if reactOpt.isSome:
  let react = reactOpt.get()
[inline-code-end]