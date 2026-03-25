## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| userId | string | query | Ne |  |

## Odziv

Vrne: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateSubscriptionAPIResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer updateSubscription'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za kakršenkoli problem prosimo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(notificationFrequency: 123) // UpdateAPIUserSubscriptionData | 
let userId = "userId_example" // String |  (neobvezno)

DefaultAPI.updateSubscription(tenantId: tenantId, id: id, updateAPIUserSubscriptionData: updateAPIUserSubscriptionData, userId: userId) { (response, error) in
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