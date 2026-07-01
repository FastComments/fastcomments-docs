---
Aktuell online betrachter einer Seite: Personen, deren Websocket‑Sitzung gerade die Seite abonniert hat.  
Gibt anonCount + totalCount zurück (räumweite Abonnenten, einschließlich anonymer Betrachter, die wir nicht aufzählen).

## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Seiten-URL-Bezeichner (serverseitig bereinigt). |
| afterName | string | query | No | Cursor: nextAfterName aus der vorherigen Antwort übergeben. |
| afterUserId | string | query | No | Cursor‑Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge entfernen. |

## Response

Rückgabe: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Example

[inline-code-attrs-start title = 'Beispiel getOnlineUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch im Beta‑Stadium. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Seiten-URL-Bezeichner (serverseitig bereinigt).
let afterName = "afterName_example" // String | Cursor: nextAfterName aus der vorherigen Antwort übergeben. (optional)
let afterUserId = "afterUserId_example" // String | Cursor‑Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleichheiten keine Einträge entfernen. (optional)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
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