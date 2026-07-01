## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| meta | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`GetTenantsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTenantsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getTenants'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. Za vsak problem, prosimo, poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let meta = "meta_example" // String |  (neobvezno)
let skip = 987 // Double |  (neobvezno)

DefaultAPI.getTenants(tenantId: tenantId, options: DefaultAPI.GetTenantsOptions(meta: meta, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]