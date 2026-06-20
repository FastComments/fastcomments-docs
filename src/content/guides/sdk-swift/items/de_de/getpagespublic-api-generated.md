Listet Seiten für einen Mandanten. Wird vom FChat-Desktop-Client verwendet, um dessen Raumliste zu füllen. Erfordert, dass `enableFChat` in der aufgelösten benutzerdefinierten Konfiguration für jede Seite auf true gesetzt ist. Seiten, die SSO erfordern, werden anhand des Gruppen-Zugriffs des anfragenden Benutzers gefiltert.

## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| cursor | string | query | Nein | Opaker Paginierungscursor, der als `nextCursor` aus einer vorherigen Anfrage zurückgegeben wurde. Ist an denselben `sortBy` gebunden. |
| limit | integer | query | Nein | 1..200, Standard 50 |
| q | string | query | Nein | Optionaler Titel-Präfixfilter ohne Berücksichtigung der Groß-/Kleinschreibung. |
| sortBy | string | query | Nein | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst), oder `title` (alphabetisch). |
| hasComments | boolean | query | Nein | Wenn true, werden nur Seiten zurückgegeben, die mindestens einen Kommentar haben. |

## Antwort

Gibt zurück: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPublicPagesResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getPagesPublic Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie sich bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let cursor = "cursor_example" // String | Opaker Paginierungscursor, der als `nextCursor` aus einer vorherigen Anfrage zurückgegeben wurde. Ist an denselben `sortBy` gebunden. (optional)
let limit = 987 // Int | 1..200, Standard 50 (optional)
let q = "q_example" // String | Optionaler Titel-Präfixfilter ohne Berücksichtigung der Groß-/Kleinschreibung. (optional)
let sortBy = PagesSortBy() // PagesSortBy | Sortierreihenfolge. `updatedAt` (Standard, neueste zuerst), `commentCount` (meiste Kommentare zuerst), oder `title` (alphabetisch). (optional)
let hasComments = true // Bool | Wenn true, werden nur Seiten zurückgegeben, die mindestens einen Kommentar haben. (optional)

PublicAPI.getPagesPublic(tenantId: tenantId, cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]