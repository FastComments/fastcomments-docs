## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| options | GetSearchSitesOptions | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[ModerationSiteSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_site_search_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getSearchSites उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (searchResult, httpResp) = client.getSearchSites(tenantId = "my-tenant-123", options = GetSearchSitesOptions())
if searchResult.isSome:
  let siteResp = searchResult.get()
[inline-code-end]