---
## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| urlId | string | Evet |  |
| id | string | Hayır |  |

## Yanıt

Döndürür: [`Option[GetV2PageReactUsersResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_react_users_response.nim)

## Örnek

[inline-code-attrs-start title = 'getV2PageReactUsers Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV2PageReactUsers(tenantId = "my-tenant-123", urlId = "news/article-title", id = "")
if response.isSome:
  let usersResp = response.get()
  echo repr(usersResp)
else:
  echo "No page react users returned. HTTP response: ", repr(httpResponse)
[inline-code-end]

---