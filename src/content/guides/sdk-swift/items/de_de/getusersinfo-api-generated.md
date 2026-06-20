Massenbenutzerinformationen für einen Mandanten. Anhand von userIds werden Anzeigeinformationen aus User / SSOUser zurückgegeben.
Wird vom Kommentar-Widget verwendet, um Benutzer anzureichern, die gerade über ein Präsenzereignis erschienen sind.
Kein Seitenkontext: Der Datenschutz wird einheitlich durchgesetzt (private Profile werden maskiert).

## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| ids | string | query | Ja | Kommagetrennte userIds. |

## Antwort

Gibt zurück: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersInfoResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getUsersInfo Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta. Bei Problemen melden Sie dies bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let ids = "ids_example" // String | Kommagetrennte userIds.

PublicAPI.getUsersInfo(tenantId: tenantId, ids: ids) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]