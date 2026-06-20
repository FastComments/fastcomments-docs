---
Frühere Kommentierende auf der Seite, die derzeit NICHT online sind. Sortiert nach displayName.
Verwenden Sie dies, nachdem Sie /users/online erschöpft haben, um einen "Mitglieder"-Abschnitt darzustellen.
Cursor-Pagination auf commenterName: der Server durchläuft den partiellen {tenantId, urlId, commenterName}-Index von afterName vorwärts mittels $gt, ohne $skip-Kosten.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Seiten-URL-Identifikator (serverseitig bereinigt). |
| afterName | string | query | No | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | No | Cursor-Entscheider: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Einträge bei Namensgleichheit nicht verloren gehen. |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getOfflineUsers Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | Seiten-URL-Identifikator (serverseitig bereinigt).
let afterName = "afterName_example" // String | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. (optional)
let afterUserId = "afterUserId_example" // String | Cursor-Entscheider: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Einträge bei Namensgleichheit nicht verloren gehen. (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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