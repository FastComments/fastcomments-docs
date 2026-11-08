## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| namespace | string | नहीं |  |
| component | string | नहीं |  |
| options | GetTranslationsOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetTranslationsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_translations_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getTranslations उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetTranslationsOptions()
let (maybeResp, httpResp) = client.getTranslations(namespace = "my-tenant-123", component = "news/article-title", options = opts)
if maybeResp.isSome:
  let resp = maybeResp.get()
  echo resp
[inline-code-end]