## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| id | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GetV2PageReactUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_react_users_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getV2PageReactUsers उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResponse, httpResponse) = client.getV2PageReactUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  id = "user-456"
)

if maybeResponse.isSome:
  let resp = maybeResponse.get()
  echo resp
[inline-code-end]