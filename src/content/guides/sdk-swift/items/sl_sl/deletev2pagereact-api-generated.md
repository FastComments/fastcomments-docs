## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| id | string | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Primer

[inline-code-attrs-start title = 'deleteV2PageReact Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za morebitne težave prosimo, da jih prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let id = "id_example" // String | 
let sso = "sso_example" // String |  (neobvezno)

PublicAPI.deleteV2PageReact(tenantId: tenantId, urlId: urlId, id: id, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---