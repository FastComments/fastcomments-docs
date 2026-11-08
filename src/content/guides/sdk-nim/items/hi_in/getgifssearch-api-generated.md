## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| search | string | नहीं |  |
| options | GetGifsSearchOptions | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[GetGifsSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_gifs_search_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getGifsSearch उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetGifsSearchOptions(limit = 10, rating = "g")
let (responseOpt, httpResponse) = client.getGifsSearch(tenantId = "my-tenant-123", search = "funny cats", options = opts)
if responseOpt.isSome:
  let resp = responseOpt.get()
  # resp का आवश्यकतानुसार उपयोग करें
[inline-code-end]