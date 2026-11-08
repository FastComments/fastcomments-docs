## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetSearchSuggestOptions | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`Option[ModerationSuggestResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_suggest_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getSearchSuggest उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (suggestOpt, httpResp) = client.getSearchSuggest(
  tenantId = "my-tenant-123",
  options = GetSearchSuggestOptions(),
)

if suggestOpt.isSome:
  let suggest = suggestOpt.get()
  echo suggest
[inline-code-end]