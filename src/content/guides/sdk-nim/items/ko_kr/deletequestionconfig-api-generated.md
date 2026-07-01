## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'deleteQuestionConfig 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResult, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "question-config-456")
if apiResult.isSome:
  let empty = apiResult.get()
[inline-code-end]