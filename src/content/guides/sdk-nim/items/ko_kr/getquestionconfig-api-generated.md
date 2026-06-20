---
## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |

## 응답

반환: [`Option[GetQuestionConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config_response.nim)

## 예제

[inline-code-attrs-start title = 'getQuestionConfig 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "qst-456")
if response.isSome:
  let cfg = response.get()
  discard cfg
[inline-code-end]

---