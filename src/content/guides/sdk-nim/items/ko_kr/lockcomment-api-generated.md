## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'lockComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.lockComment(tenantId = "news-tenant-42", commentId = "cmt-8f3a2b9d", broadcastId = "", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Locked comment successfully for tenant news-tenant-42"
else:
  echo "Failed to lock comment, HTTP status: ", $httpResponse.status
[inline-code-end]

---