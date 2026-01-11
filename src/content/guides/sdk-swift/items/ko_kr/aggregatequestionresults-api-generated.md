## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| questionId | string | query | 아니요 |  |
| questionIds | array | query | 아니요 |  |
| urlId | string | query | 아니요 |  |
| timeBucket | string | query | 아니요 |  |
| startDate | string | query | 아니요 |  |
| forceRecalculate | boolean | query | 아니요 |  |

## 응답

반환: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## 예제

[inline-code-attrs-start title = 'aggregateQuestionResults 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (선택 사항)
let questionIds = ["inner_example"] // [String] |  (선택 사항)
let urlId = "urlId_example" // String |  (선택 사항)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (선택 사항)
let startDate = Date() // Date |  (선택 사항)
let forceRecalculate = true // Bool |  (선택 사항)

DefaultAPI.aggregateQuestionResults(tenantId: tenantId, questionId: questionId, questionIds: questionIds, urlId: urlId, timeBucket: timeBucket, startDate: startDate, forceRecalculate: forceRecalculate) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]