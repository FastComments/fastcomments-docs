## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
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

## 예시

[inline-code-attrs-start title = 'getQuestionResults 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 를 통해 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (옵션)
let userId = "userId_example" // String |  (옵션)
let startDate = "startDate_example" // String |  (옵션)
let questionId = "questionId_example" // String |  (옵션)
let questionIds = "questionIds_example" // String |  (옵션)
let skip = 987 // Double |  (옵션)

DefaultAPI.getQuestionResults(tenantId: tenantId, options: DefaultAPI.GetQuestionResultsOptions(urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]