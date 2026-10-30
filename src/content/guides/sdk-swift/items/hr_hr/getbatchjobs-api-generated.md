The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Samo poslovi ovog tipa, na primjer migrate-pages. |
| status | string | query | No | Samo poslovi u ovom statusu: requested, running, success or failed. |
| skip | integer | query | No | Koliko poslova preskočiti, za paginaciju. |

## Odgovor

Vraća: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još beta. Za bilo koji problem, molimo prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Samo poslovi ovog tipa, na primjer migrate-pages. (opcionalno)
let status = "status_example" // String | Samo poslovi u ovom statusu: requested, running, success or failed. (opcionalno)
let skip = 987 // Int | Koliko poslova preskočiti, za paginaciju. (opcionalno)

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