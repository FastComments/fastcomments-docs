## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| includeByUserIdAndEmail | boolean | query | Não |  |
| includeByIP | boolean | query | Não |  |
| includeByEmailDomain | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BulkPreBanSummary.swift)

## Exemplo

[inline-code-attrs-start title = 'postBulkPreBanSummary Exemplo'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda são beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let bulkPreBanParams = BulkPreBanParams(commentIds: ["commentIds_example"]) // BulkPreBanParams | 
let includeByUserIdAndEmail = true // Bool |  (opcional)
let includeByIP = true // Bool |  (opcional)
let includeByEmailDomain = true // Bool |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.postBulkPreBanSummary(tenantId: tenantId, bulkPreBanParams: bulkPreBanParams, options: ModerationAPI.PostBulkPreBanSummaryOptions(includeByUserIdAndEmail: includeByUserIdAndEmail, includeByIP: includeByIP, includeByEmailDomain: includeByEmailDomain, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]