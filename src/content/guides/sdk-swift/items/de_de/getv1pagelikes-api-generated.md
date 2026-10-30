## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Wenn gesetzt, werden die Benutzer zurückgegeben, die die Seite geliked haben (Avatare und Benutzernamen). Nur "public" wird akzeptiert, wodurch private Profile weggelassen werden. |

## Antwort

Rückgabe: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Beispiel

[inline-code-attrs-start title = 'getV1PageLikes Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Wenn gesetzt, werden die Benutzer zurückgegeben, die die Seite geliked haben (Avatare und Benutzernamen). Nur \"public\" wird akzeptiert, wodurch private Profile weggelassen werden. (optional)

PublicAPI.getV1PageLikes(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV1PageLikesOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]