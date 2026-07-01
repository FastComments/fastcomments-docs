## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'logoutPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (responseOpt, httpResponse) = client.logoutPublic(tenantId = "my-tenant-123", userId = "user-456")
if responseOpt.isSome:
  let emptyResponse = responseOpt.get()
[inline-code-end]