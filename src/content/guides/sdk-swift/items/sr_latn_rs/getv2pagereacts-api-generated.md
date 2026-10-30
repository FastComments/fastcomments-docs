## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Kada je postavljeno, vraća korisnike za svaki react id (avatari i korisnička imena). Prihvaća se samo \"public\", što izostavlja privatne profile. |

## Odgovor

Vraća: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## Primer

[inline-code-attrs-start title = 'Primer getV2PageReacts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su i dalje beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (opciono)
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | Kada je postavljeno, vraća korisnike za svaki react id (avatari i korisnička imena). Prihvaća se samo \"public\", što izostavlja privatne profile. (opciono)

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