## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |

## 응답

반환: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateQuestionConfigResponse.swift)

## 예시

[inline-code-attrs-start title = 'createQuestionConfig 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 에서 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createQuestionConfigBody = CreateQuestionConfigBody(name: "name_example", question: "question_example", helpText: "helpText_example", type: "type_example", numStars: 123, min: 123, max: 123, defaultValue: 123, labelNegative: "labelNegative_example", labelPositive: "labelPositive_example", customOptions: [QuestionConfig_customOptions_inner(imageSrc: "imageSrc_example", name: "name_example")], subQuestionIds: ["subQuestionIds_example"], alwaysShowSubQuestions: false, subQuestionConditions: [SubQuestionCondition(_operator: QuestionConditionOperator(), value: 123, subQuestionId: "subQuestionId_example")], commentBoxCondition: QuestionCondition(_operator: nil, value: 123), commentBoxPrompt: "commentBoxPrompt_example", reportingOrder: 123) // CreateQuestionConfigBody | 

DefaultAPI.createQuestionConfig(tenantId: tenantId, createQuestionConfigBody: createQuestionConfigBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]