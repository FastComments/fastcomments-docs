## Parameters

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| batchJobId | string | query | Não |  |
| sso | string | query | Não |  |

## Response

Returns: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportStatusResponse.swift)

## Example

[inline-code-attrs-start title = 'Exemplo getApiExportStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// As amostras de código a seguir ainda estão em beta. Para quaisquer problemas, por favor informe via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let batchJobId = "batchJobId_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.getApiExportStatus(tenantId: tenantId, options: ModerationAPI.GetApiExportStatusOptions(batchJobId: batchJobId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]