---
## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예제

[inline-code-attrs-start title = 'deleteQuestionResult 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let resultId = "question-result-456"
let (response, httpResponse) = client.deleteQuestionResult(tenantId = tenantId, id = resultId)
if response.isSome:
  let emptyResp = response.get()
  echo "Deleted question result:", resultId
[inline-code-end]

---