## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| questionId | string | query | 아니오 |  |
| questionIds | array | query | 아니오 |  |
| urlId | string | query | 아니오 |  |
| startDate | string | query | 아니오 |  |
| forceRecalculate | boolean | query | 아니오 |  |
| minValue | number | query | 아니오 |  |
| maxValue | number | query | 아니오 |  |
| limit | number | query | 아니오 |  |

## 응답

반환: [`CombineQuestionResultsWithCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CombineQuestionResultsWithCommentsResponse.swift)

## 예제

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있는 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 을 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (선택 사항)
let questionIds = ["inner_example"] // [String] |  (선택 사항)
let urlId = "urlId_example" // String |  (선택 사항)
let startDate = Date() // Date |  (선택 사항)
let forceRecalculate = true // Bool |  (선택 사항)
let minValue = 987 // Double |  (선택 사항)
let maxValue = 987 // Double |  (선택 사항)
let limit = 987 // Double |  (선택 사항)

DefaultAPI.combineCommentsWithQuestionResults(tenantId: tenantId, questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit) { (response, error) in
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