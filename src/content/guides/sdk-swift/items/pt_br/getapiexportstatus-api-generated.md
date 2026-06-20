## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportStatusResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getApiExportStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let batchJobId = "batchJobId_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.getApiExportStatus(batchJobId: batchJobId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]