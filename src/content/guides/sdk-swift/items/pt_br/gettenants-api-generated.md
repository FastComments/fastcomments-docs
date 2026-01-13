## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| meta | string | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenants200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getTenants'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os seguintes exemplos de código ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let meta = "meta_example" // String |  (opcional)
let skip = 987 // Double |  (opcional)

DefaultAPI.getTenants(tenantId: tenantId, meta: meta, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]