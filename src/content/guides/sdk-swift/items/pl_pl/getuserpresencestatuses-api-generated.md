## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| urlIdWS | string | query | Tak |  |
| userIds | string | query | Tak |  |

## Odpowiedź

Zwraca: [`GetUserPresenceStatusesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserPresenceStatusesResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getUserPresenceStatuses'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlIdWS = "urlIdWS_example" // String | 
let userIds = "userIds_example" // String | 

PublicAPI.getUserPresenceStatuses(tenantId: tenantId, urlIdWS: urlIdWS, userIds: userIds) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]