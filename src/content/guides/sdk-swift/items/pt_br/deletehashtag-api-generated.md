---
## Parâmetros

| Name | Type | Location | Obrigatório | Description |
|------|------|----------|-------------|-------------|
| tenantId | string | query | Sim |  |
| tag | string | path | Sim |  |

## Resposta

Retorna: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'deleteHashTag Exemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// O código de exemplo a seguir ainda está em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let tag = "tag_example" // String | 
let deleteHashTagRequestBody = DeleteHashTagRequestBody(tenantId: "tenantId_example") // DeleteHashTagRequestBody |  (optional)

DefaultAPI.deleteHashTag(tenantId: tenantId, tag: tag, deleteHashTagRequestBody: deleteHashTagRequestBody) { (response, error) in
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