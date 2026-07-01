## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## 回應

返回：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'deleteQuestionConfig 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResult, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "question-config-456")
if apiResult.isSome:
  let empty = apiResult.get()
[inline-code-end]