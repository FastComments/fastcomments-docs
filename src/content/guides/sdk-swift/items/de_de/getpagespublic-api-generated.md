Listet Seiten für einen Tenant auf. Wird vom FChat-Desktop-Client verwendet, um seine Raumliste zu füllen.
Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist.
Seiten, die SSO erfordern, werden gegen den Gruppenzugriff des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nein | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An denselben `sortBy` gebunden. |
| limit | integer | query | Nein | 1..200, Standard 50 |
| q | string | query | Nein | Optionaler Filter für Titel-Präfix, Groß-/Kleinschreibung wird nicht beachtet. |
| sortBy | string | query | Nein | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). |
| hasComments | boolean | query | Nein | Wenn true, werden nur Seiten mit mindestens einem Kommentar zurückgegeben. |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Opaker Paginierungs-Cursor, der als `nextCursor` von einer vorherigen Anfrage zurückgegeben wurde. An denselben `sortBy` gebunden. (optional)
let limit = 987 // Int | 1..200, Standard 50 (optional)
let q = "q_example" // String | Optionaler Filter für Titel-Präfix, Groß-/Kleinschreibung wird nicht beachtet. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst) oder `title` (alphabetisch). (optional)
let hasComments = true // Bool | Wenn true, werden nur Seiten mit mindestens einem Kommentar zurückgegeben. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, options: PublicAPI.GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]