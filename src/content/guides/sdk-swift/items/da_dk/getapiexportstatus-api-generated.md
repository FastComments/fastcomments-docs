## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Ja |  |
| batchJobId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportStatusResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getApiExportStatus Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapportér venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let batchJobId = "batchJobId_example" // String |  (valgfri)
let sso = "sso_example" // String |  (valgfri)

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