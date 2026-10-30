## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Ko je nastavljeno, vrne uporabnike, ki so všečkali stran (avatari in uporabniška imena). Sprejema se le "public", kar izpusti zasebne profile. |

## Odgovor

Vrne: [`GetV1PageLikes`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV1PageLikes.swift)

## Primer

[inline-code-attrs-start title = 'Primer getV1PageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za morebitne težave prosimo, da jih prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (optional)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Ko je nastavljeno, vrne uporabnike, ki so všečkali stran (avatari in uporabniška imena). Sprejema se le "public", kar izpusti zasebne profile. (optional)

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