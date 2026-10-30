The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Samo poslovi ovog tipa, na primer migrate-pages. |
| status | string | query | No | Samo poslovi u ovom statusu: requested, running, success ili failed. |
| skip | integer | query | No | Koliko poslova treba preskočiti, za paginaciju. |

## Odgovor

Vraća: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći kod primeri su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Samo poslovi ovog tipa, na primer migrate-pages. (opciono)
let status = "status_example" // String | Samo poslovi u ovom statusu: requested, running, success ili failed. (opciono)
let skip = 987 // Int | Koliko poslova treba preskočiti, za paginaciju. (opciono)

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