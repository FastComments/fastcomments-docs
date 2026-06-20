## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odziv

Vrača: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ResetUserNotificationsResponse.swift)

## Primer

[inline-code-attrs-start title = 'resetUserNotificationCount Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Če naletite na kakršno koli težavo, jo prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let sso = "sso_example" // String |  (neobvezno)

PublicAPI.resetUserNotificationCount(tenantId: tenantId, sso: sso) { (response, error) in
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