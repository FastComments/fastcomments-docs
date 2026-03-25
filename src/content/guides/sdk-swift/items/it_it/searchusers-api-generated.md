---
## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì |  |
| usernameStartsWith | string | query | No |  |
| mentionGroupIds | array | query | No |  |
| sso | string | query | No |  |
| searchSection | string | query | No |  |

## Risposta

Restituisce: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SearchUsers200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di searchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per eventuali problemi, segnalarli su http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let usernameStartsWith = "usernameStartsWith_example" // String |  (opzionale)
let mentionGroupIds = ["inner_example"] // [String] |  (opzionale)
let sso = "sso_example" // String |  (opzionale)
let searchSection = "searchSection_example" // String |  (opzionale)

PublicAPI.searchUsers(tenantId: tenantId, urlId: urlId, usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection) { (response, error) in
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