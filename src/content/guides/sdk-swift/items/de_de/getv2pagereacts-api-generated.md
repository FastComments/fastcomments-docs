## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| sso | string | query | Nein |  |
| includeUserInfo | string | query | Nein | Wenn gesetzt, werden die Benutzer für jede Reaktions‑ID zurückgegeben (Avatare und Benutzernamen). Nur "public" wird akzeptiert, wodurch private Profile weggelassen werden. |

## Rückgabe

Rückgabe: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Beispiel

[inline-code-attrs-start title = 'getV2PageReacts Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch in der Beta‑Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Wenn gesetzt, werden die Benutzer für jede Reaktions‑ID zurückgegeben (Avatare und Benutzernamen). Nur \"public\" wird akzeptiert, wodurch private Profile weggelassen werden. (optional)

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]