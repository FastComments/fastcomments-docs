## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| id | string | नहीं |  |
| title | string = "" | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[CreateV1PageReact]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_v1_page_react.nim)

## उदाहरण

[inline-code-attrs-start title = 'createV2PageReact उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pageResult, httpResponse) = client.createV2PageReact(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  id = "page-456",
  title = "Breaking News",
)

if pageResult.isSome:
  let page = pageResult.get()
  # `page` का आवश्यकतानुसार उपयोग करें
[inline-code-end]