## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No | Utilizzato per determinare se la pagina corrente è iscritta. |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| includeTenantNotifications | boolean | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMyNotificationsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio getUserNotifications'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Il seguente esempio di codice è ancora in beta. Per qualsiasi problema, segnalalo tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let urlId = "urlId_example" // String | Utilizzato per determinare se la pagina corrente è iscritta. (opzionale)
let pageSize = 987 // Int |  (opzionale)
let afterId = "afterId_example" // String |  (opzionale)
let includeContext = true // Bool |  (opzionale)
let afterCreatedAt = 987 // Int64 |  (opzionale)
let unreadOnly = true // Bool |  (opzionale)
let dmOnly = true // Bool |  (opzionale)
let noDm = true // Bool |  (opzionale)
let includeTranslations = true // Bool |  (opzionale)
let includeTenantNotifications = true // Bool |  (opzionale)
let sso = "sso_example" // String |  (opzionale)

PublicAPI.getUserNotifications(tenantId: tenantId, options: PublicAPI.GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]