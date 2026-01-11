## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateSubscriptionAPIResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'createSubscription Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du oplever problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createAPIUserSubscriptionData = CreateAPIUserSubscriptionData(pageTitle: "pageTitle_example", url: "url_example", urlId: "urlId_example", anonUserId: "anonUserId_example", userId: "userId_example") // CreateAPIUserSubscriptionData | 

DefaultAPI.createSubscription(tenantId: tenantId, createAPIUserSubscriptionData: createAPIUserSubscriptionData) { (response, error) in
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