## パラメータ

| 名前 | タイプ | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| options | CombineCommentsWithQuestionResultsOptions | いいえ |  |

## レスポンス

返却: [`Option[CombineQuestionResultsWithCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_combine_question_results_with_comments_response.nim)

## 例

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (combineOpt, httpResponse) = client.combineCommentsWithQuestionResults(
  tenantId = "my-tenant-123",
  options = default(CombineCommentsWithQuestionResultsOptions)
)

if combineOpt.isSome:
  let combineResult = combineOpt.get()
[inline-code-end]