## प्रतिक्रिया

रिटर्न करता है: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'logoutPublic उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.logoutPublic(tenantId = "my-tenant-123", userId = "user-456")
if responseOpt.isSome:
  let emptyResponse = responseOpt.get()
[inline-code-end]