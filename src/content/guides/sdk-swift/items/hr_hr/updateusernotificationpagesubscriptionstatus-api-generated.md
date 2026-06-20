Omogućite ili onemogućite obavijesti za stranicu. Kada su korisnici pretplaćeni na stranicu, obavijesti se stvaraju za nove korijenske komentare, i također

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| url | string | query | Da |  |
| pageTitle | string | query | Da |  |
| subscribedOrUnsubscribed | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationPageSubscriptionStatusResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer updateUserNotificationPageSubscriptionStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let url = "url_example" // String | 
let pageTitle = "pageTitle_example" // String | 
let subscribedOrUnsubscribed = "subscribedOrUnsubscribed_example" // String | 
let sso = "sso_example" // String |  (neobavezno)

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