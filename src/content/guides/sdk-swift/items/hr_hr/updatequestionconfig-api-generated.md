## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer updateQuestionConfig'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateQuestionConfigBody = UpdateQuestionConfigBody(name: "name_example", question: "question_example", helpText: "helpText_example", type: "type_example", numStars: 123, min: 123, max: 123, defaultValue: 123, labelNegative: "labelNegative_example", labelPositive: "labelPositive_example", customOptions: [QuestionConfig_customOptions_inner(imageSrc: "imageSrc_example", name: "name_example")], subQuestionIds: ["subQuestionIds_example"], alwaysShowSubQuestions: false, reportingOrder: 123) // UpdateQuestionConfigBody | 

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