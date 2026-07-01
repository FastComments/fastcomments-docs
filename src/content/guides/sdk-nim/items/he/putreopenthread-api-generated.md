## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| urlId | string | כן |  |
| sso | string = "" | לא |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת putReopenThread'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.putReopenThread(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  sso = ""
)

if apiRespOpt.isSome:
  let emptyResp = apiRespOpt.get()
  discard
[inline-code-end]