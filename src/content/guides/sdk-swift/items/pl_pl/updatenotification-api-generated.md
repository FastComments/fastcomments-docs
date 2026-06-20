## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| id | string | path | Tak |  |
| userId | string | query | Nie |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIEmptyResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład updateNotification'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w fazie beta. W przypadku problemu prosimy zgłosić przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateNotificationBody = UpdateNotificationBody(viewed: false, optedOut: false) // UpdateNotificationBody | 
let userId = "userId_example" // String |  (opcjonalne)

DefaultAPI.updateNotification(tenantId: tenantId, id: id, updateNotificationBody: updateNotificationBody, userId: userId) { (response, error) in
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