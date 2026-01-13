## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Resposta

Retorna: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionConfig200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getQuestionConfig'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os seguintes exemplos de código ainda estão em versão beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.getQuestionConfig(tenantId: tenantId, id: id) { (response, error) in
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