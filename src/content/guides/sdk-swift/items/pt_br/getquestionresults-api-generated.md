## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| tenantId | string | query | Sim |  |
| urlId | string | query | Não |  |
| userId | string | query | Não |  |
| startDate | string | query | Não |  |
| questionId | string | query | Não |  |
| questionIds | string | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResults200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getQuestionResults'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os seguintes exemplos de código ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String |  (optional)
let userId = "userId_example" // String |  (optional)
let startDate = "startDate_example" // String |  (optional)
let questionId = "questionId_example" // String |  (optional)
let questionIds = "questionIds_example" // String |  (optional)
let skip = 987 // Double |  (optional)

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