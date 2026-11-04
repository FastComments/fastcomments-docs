Frühere Kommentatoren auf der Seite, die NICHT aktuell online sind. Sortiert nach displayName.
Verwenden Sie dies, nachdem /users/online ausgeschöpft wurde, um einen „Members“-Abschnitt darzustellen.
Cursor-Paginierung über commenterName: Der Server durchläuft den partiellen {tenantId, urlId, commenterName}
Index ab afterName vorwärts mittels $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten-URL-Identifikator (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: nextAfterName aus der vorherigen Antwort übergeben. |
| afterUserId | string | query | Nein | Cursor-Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleiche keine Einträge verlieren. |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Seiten-URL-Identifikator (serverseitig bereinigt).
let afterName = "afterName_example" // String | Cursor: nextAfterName aus der vorherigen Antwort übergeben. (optional)
let afterUserId = "afterUserId_example" // String | Cursor-Tiebreaker: nextAfterUserId aus der vorherigen Antwort übergeben. Erforderlich, wenn afterName gesetzt ist, damit Namensgleiche keine Einträge verlieren. (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]