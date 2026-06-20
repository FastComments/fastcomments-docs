Aktuell online befindliche Betrachter einer Seite: Personen, deren WebSocket-Sitzung derzeit auf die Seite abonniert ist.
Gibt anonCount + totalCount zurück (raumweite Abonnenten, einschließlich anonymer Betrachter, die wir nicht einzeln auflisten).

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Page URL identifier (cleaned server-side). |
| afterName | string | query | Nein | Cursor: pass nextAfterName from the previous response. |
| afterUserId | string | query | Nein | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries. |

## Antwort

Gibt zurück: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getOnlineUsers Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new melden
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Bezeichner der Seiten-URL (serverseitig bereinigt).
let afterName = "afterName_example" // String | Cursor: Übergib nextAfterName aus der vorherigen Antwort. (optional)
let afterUserId = "afterUserId_example" // String | Cursor-Tiebreaker: Übergib nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Einträge bei Namensgleichheit nicht entfallen. (optional)

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]