## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| limit | number | query | Não |  |
| skip | number | query | Não |  |
| order | string | query | Não |  |
| after | number | query | Não |  |
| before | number | query | Não |  |

## Resposta

Retorna: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogs200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let limit = 987 // Double |  (opcional)
let skip = 987 // Double |  (opcional)
let order = SORT_DIR() // SORTDIR |  (opcional)
let after = 987 // Double |  (opcional)
let before = 987 // Double |  (opcional)

DefaultAPI.getAuditLogs(tenantId: tenantId, limit: limit, skip: skip, order: order, after: after, before: before) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]