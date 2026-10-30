The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| type | string | query | Ne | Samo posli te vrste, na primer migrate-pages. |
| status | string | query | Ne | Samo posli v tem stanju: requested, running, success ali failed. |
| skip | integer | query | Ne | Koliko poslov naj se preskoči, za straničenje. |

## Response

Vrne: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'getBatchJobs Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za morebitne težave, prosimo, poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Samo posli te vrste, na primer migrate-pages. (neobvezno)
let status = "status_example" // String | Samo posli v tem stanju: requested, running, success ali failed. (neobvezno)
let skip = 987 // Int | Koliko poslov naj se preskoči, za straničenje. (neobvezno)

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