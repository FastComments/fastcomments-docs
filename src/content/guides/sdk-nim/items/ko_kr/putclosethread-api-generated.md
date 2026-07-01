## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| sso | string = "" | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'putCloseThread 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.putCloseThread(tenantId = "my-tenant-123", urlId = "news/fastcomments-integration", sso = "")
if respOpt.isSome:
  let empty = respOpt.get()
  echo "Thread successfully closed"
[inline-code-end]