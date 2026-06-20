---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| urlId | string | 예 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'putCloseThread 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putCloseThread(urlId = "news/article-title", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Thread closed successfully:", $apiResp
else:
  echo "Failed to close thread, HTTP response:", $httpResponse
[inline-code-end]

---