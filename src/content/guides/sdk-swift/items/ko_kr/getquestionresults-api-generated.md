## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| urlId | string | query | 아니오 |  |
| userId | string | query | 아니오 |  |
| startDate | string | query | 아니오 |  |
| questionId | string | query | 아니오 |  |
| questionIds | string | query | 아니오 |  |
| skip | number | query | 아니오 |  |

## 응답

반환: [`GetQuestionResultsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResultsResponse.swift)

## 예제

[inline-code-attrs-start title = 'getQuestionResults 예제'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타입니다. 문제가 있을 경우 http://github.com/OpenAPITools/openapi-generator/issues/new 에서 신고해주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (선택 사항)
let userId = "userId_example" // String |  (선택 사항)
let startDate = "startDate_example" // String |  (선택 사항)
let questionId = "questionId_example" // String |  (선택 사항)
let questionIds = "questionIds_example" // String |  (선택 사항)
let skip = 987 // Double |  (선택 사항)

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