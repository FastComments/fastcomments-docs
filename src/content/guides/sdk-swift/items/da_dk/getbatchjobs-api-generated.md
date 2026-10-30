Lejers batch-job (import, eksport, side‑migrationer...), nyeste først.

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| type | string | query | Nej | Kun job af denne type, for eksempel migrate-pages. |
| status | string | query | Nej | Kun job i denne status: requested, running, success eller failed. |
| skip | integer | query | Nej | Hvor mange job der skal springes over, for sideinddeling. |

## Svar

Returnerer: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getBatchJobs Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Kun job af denne type, for eksempel migrate-pages. (valgfri)
let status = "status_example" // String | Kun job i denne status: requested, running, success eller failed. (valgfri)
let skip = 987 // Int | Hvor mange job der skal springes over, for sideinddeling. (valgfri)

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