The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| type | string | query | Não | Somente jobs deste tipo, por exemplo migrate-pages. |
| status | string | query | Não | Somente jobs neste status: requested, running, success ou failed. |
| skip | integer | query | Não | Quantos jobs pular, para paginação. |

## Resposta

Retorna: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Somente jobs deste tipo, por exemplo migrate-pages. (opcional)
let status = "status_example" // String | Somente jobs neste status: requested, running, success ou failed. (opcional)
let skip = 987 // Int | Quantos jobs pular, para paginação. (opcional)

DefaultAPI.getBatchJobs(tenantId: tenantId, options: DefaultAPI.GetBatchJobsOptions(type: type, status: status, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]