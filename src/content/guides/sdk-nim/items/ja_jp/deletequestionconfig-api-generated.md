---
## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## レスポンス

戻り値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'deleteQuestionConfig の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResult, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "question-config-456")
if apiResult.isSome:
  let empty = apiResult.get()
[inline-code-end]

---