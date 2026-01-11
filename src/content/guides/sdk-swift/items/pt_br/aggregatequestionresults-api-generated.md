## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| questionId | string | query | Não |  |
| questionIds | array | query | Não |  |
| urlId | string | query | Não |  |
| timeBucket | string | query | Não |  |
| startDate | string | query | Não |  |
| forceRecalculate | boolean | query | Não |  |

## Resposta

Retorna: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let questionId = "questionId_example" // String |  (opcional)
let questionIds = ["inner_example"] // [String] |  (opcional)
let urlId = "urlId_example" // String |  (opcional)
let timeBucket = AggregateTimeBucket() // AggregateTimeBucket |  (opcional)
let startDate = Date() // Date |  (opcional)
let forceRecalculate = true // Bool |  (opcional)

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