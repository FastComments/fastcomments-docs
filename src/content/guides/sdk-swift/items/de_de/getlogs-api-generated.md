## Parameter

| Name      | Typ    | Ort   | Erforderlich | Beschreibung |
|-----------|--------|-------|--------------|--------------|
| tenantId  | string | query | Ja           |  |
| commentId | string | path  | Ja           |  |
| sso       | string | query | Nein         |  |

## Antwort

Rückgabe: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetLogsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'Beispiel getLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch im Beta‑Status. Bei Problemen melden Sie diese bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (optional)

ModerationAPI.getLogs(tenantId: tenantId, commentId: commentId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]