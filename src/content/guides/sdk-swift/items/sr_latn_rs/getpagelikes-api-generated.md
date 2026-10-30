Broj lajkova za stranicu, opcionalno sa korisnicima koji su je lajkovali.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| urlId | string | query | Da |  |
| includeUserInfo | string | query | Ne | "\"public\" izostavlja korisnike sa privatnim profilima, \"include-private\" ih uključuje." |

## Response

Returns: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPageLikesAPIResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getPageLikes'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
```swift
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | \"public\" izostavlja korisnike sa privatnim profilima, \"include-private\" ih uključuje. (opciono)

DefaultAPI.getPageLikes(tenantId: tenantId, urlId: urlId, includeUserInfo: includeUserInfo) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```
[inline-code-end]

---