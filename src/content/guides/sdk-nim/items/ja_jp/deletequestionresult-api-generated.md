## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | いいえ |  |

## レスポンス

返却: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 例

[inline-code-attrs-start title = 'deleteQuestionResult の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteQuestionResult(tenantId = "my-tenant-123", id = "question-result-789")
if response.isSome:
  let result = response.get()
  echo "Deleted question result:", result
else:
  echo "No result returned, HTTP status:", $httpResponse.status
[inline-code-end]

---