## Parameters

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Ko je nastavljeno, vrne uporabnike za vsak react id (avatari in uporabniška imena). Samo \"public\" je sprejeto, kar izpusti zasebne profile. |

## Odziv

Vrne: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Primer

[inline-code-attrs-start title = 'Primer getV2PageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za morebitne težave, prosimo, poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (neobvezno)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Ko je nastavljeno, vrne uporabnike za vsak react id (avatari in uporabniška imena). Samo \"public\" je sprejeto, kar izpusti zasebne profile. (neobvezno)

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