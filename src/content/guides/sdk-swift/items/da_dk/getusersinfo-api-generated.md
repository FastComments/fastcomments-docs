---
Bulk-brugeroplysninger for en tenant. Givet userIds returneres visningsoplysninger fra User / SSOUser.
Bruges af kommentar-widgeten til at berige brugere, der netop dukkede op via en presence-begivenhed.
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler er maskeret).

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| ids | string | query | Ja | Komma-adskilte userIds. |

## Svar

Returnerer: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getUsersInfo Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | Komma-adskilte userIds.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---