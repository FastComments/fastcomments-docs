Abilita o disabilita le notifiche per una pagina. Quando gli utenti sono iscritti a una pagina, vengono create notifiche per i nuovi commenti principali, e anche

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| urlId | string | query | Sì |  |
| url | string | query | Sì |  |
| pageTitle | string | query | Sì |  |
| subscribedOrUnsubscribed | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateUserNotificationPageSubscriptionStatusResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateUserNotificationPageSubscriptionStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalarlo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let url = "url_example" // String | 
let pageTitle = "pageTitle_example" // String | 
let subscribedOrUnsubscribed = "subscribedOrUnsubscribed_example" // String | 
let sso = "sso_example" // String |  (opzionale)

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