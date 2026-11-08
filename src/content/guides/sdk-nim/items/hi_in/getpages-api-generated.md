## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getPages उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (pagesOpt, httpResp) = client.getPages(tenantId = "my-tenant-123")
if pagesOpt.isSome:
  let pages = pagesOpt.get()
  echo pages
else:
  echo "No pages returned"
echo httpResp
[inline-code-end]