## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| title | string | query | Ne |  |
| sso | string | query | Ne |  |

## Response

Vraća: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Primer

[inline-code-attrs-start title = 'createV1PageReact Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let title = "title_example" // String |  (opciono)
let sso = "sso_example" // String |  (opciono)

PublicAPI.createV1PageReact(tenantId: tenantId, urlId: urlId, options: PublicAPI.CreateV1PageReactOptions(title: title, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]