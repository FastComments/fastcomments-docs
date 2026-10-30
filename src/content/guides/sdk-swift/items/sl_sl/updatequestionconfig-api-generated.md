## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Primer

[inline-code-attrs-start title = 'updateQuestionConfig Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta različici. Za morebitne težave jih prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateQuestionConfigBody = UpdateQuestionConfigBody(name: "name_example", question: "question_example", helpText: "helpText_example", type: "type_example", numStars: 123, min: 123, max: 123, defaultValue: 123, labelNegative: "labelNegative_example", labelPositive: "labelPositive_example", customOptions: [QuestionConfig_customOptions_inner(imageSrc: "imageSrc_example", name: "name_example")], subQuestionIds: ["subQuestionIds_example"], alwaysShowSubQuestions: false, subQuestionConditions: [SubQuestionCondition(_operator: QuestionConditionOperator(), value: 123, subQuestionId: "subQuestionId_example")], commentBoxCondition: QuestionCondition(_operator: nil, value: 123), commentBoxPrompt: "commentBoxPrompt_example", reportingOrder: 123) // UpdateQuestionConfigBody | 

DefaultAPI.updateQuestionConfig(tenantId: tenantId, id: id, updateQuestionConfigBody: updateQuestionConfigBody) { (response, error) in
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