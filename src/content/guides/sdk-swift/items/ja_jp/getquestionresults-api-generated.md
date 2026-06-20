## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| urlId | string | query | いいえ |  |
| userId | string | query | いいえ |  |
| startDate | string | query | いいえ |  |
| questionId | string | query | いいえ |  |
| questionIds | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResultsResponse.swift)

## 例

[inline-code-attrs-start title = 'getQuestionResults の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (任意)
let userId = "userId_example" // String |  (任意)
let startDate = "startDate_example" // String |  (任意)
let questionId = "questionId_example" // String |  (任意)
let questionIds = "questionIds_example" // String |  (任意)
let skip = 987 // Double |  (任意)

DefaultAPI.getQuestionResults(tenantId: tenantId, urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---