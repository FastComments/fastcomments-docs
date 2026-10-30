De batchtaken van de tenant (imports, exports, paginamigraties...), nieuwste eerst.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | query | Ja |  |
| type | string | query | Nee | Alleen taken van dit type, bijvoorbeeld migrate-pages. |
| status | string | query | Nee | Alleen taken met deze status: requested, running, success of failed. |
| skip | integer | query | Nee | Hoeveel taken over te slaan, voor paginering. |

## Response

Returns: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'getBatchJobs Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in beta. Voor eventuele problemen, rapporteer via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Alleen taken van dit type, bijvoorbeeld migrate-pages. (optioneel)
let status = "status_example" // String | Alleen taken met deze status: requested, running, success of failed. (optioneel)
let skip = 987 // Int | Hoeveel taken over te slaan, voor paginering. (optioneel)

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