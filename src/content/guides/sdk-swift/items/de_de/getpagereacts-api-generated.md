Benutzerdefinierte Reaktionszähler für eine Seite, optional mit den Benutzern, die reagiert haben.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|---------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |
| includeUserInfo | string | query | Nein | "public" lässt Benutzer mit privaten Profilen aus, "include-private" schließt sie ein. |

## Antwort

Rückgabe: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageReactsAPIResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getPageReacts Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch in der Beta-Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" lässt Benutzer mit privaten Profilen aus, \"include-private\" schließt sie ein. (optional)

DefaultAPI.getPageReacts(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]