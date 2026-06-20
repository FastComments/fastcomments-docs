---
Aktivér eller deaktiver notifikationer for en side. Når brugere er tilmeldt en side, oprettes der notifikationer
for nye overordnede kommentarer, og også

## Parametre

| Navn | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| url | string | query | Ja |  |
| pageTitle | string | query | Ja |  |
| subscribedOrUnsubscribed | string | path | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationPageSubscriptionStatusResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let url = "url_example" // String | 
let pageTitle = "pageTitle_example" // String | 
let subscribedOrUnsubscribed = "subscribedOrUnsubscribed_example" // String | 
let sso = "sso_example" // String |  (valgfri)

PublicAPI.updateUserNotificationPageSubscriptionStatus(tenantId: tenantId, urlId: urlId, url: url, pageTitle: pageTitle, subscribedOrUnsubscribed: subscribedOrUnsubscribed, sso: sso) { (response, error) in
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