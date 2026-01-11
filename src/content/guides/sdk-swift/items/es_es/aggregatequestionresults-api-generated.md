## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| timeBucket | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |

## Respuesta

Devuelve: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateQuestionResults200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de aggregateQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en fase beta. Para cualquier problema, por favor repórtelo vía http://github.com/OpenAPITools/openapi-generator/issues/new
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