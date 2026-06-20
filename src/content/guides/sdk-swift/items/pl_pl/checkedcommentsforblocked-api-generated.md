---
## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| commentIds | string | query | Tak | Lista identyfikatorów komentarzy rozdzielona przecinkami. |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CheckBlockedCommentsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład checkedCommentsForBlocked'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemu zgłoś go pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentIds = "commentIds_example" // String | Lista identyfikatorów komentarzy rozdzielona przecinkami.
let sso = "sso_example" // String |  (opcjonalne)

PublicAPI.checkedCommentsForBlocked(tenantId: tenantId, commentIds: commentIds, sso: sso) { (response, error) in
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