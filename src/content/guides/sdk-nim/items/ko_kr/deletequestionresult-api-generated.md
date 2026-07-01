## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'deleteQuestionResult 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeResult, httpResp) = client.deleteQuestionResult(tenantId = "my-tenant-123", id = "question-456")
if maybeResult.isSome:
  let result = maybeResult.get()
[inline-code-end]

---