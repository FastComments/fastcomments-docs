## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |

## Odgovor

Vraća: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateV1PageReact.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer deleteV1PageReact'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda još su beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  (opcionalno)

PublicAPI.deleteV1PageReact(tenantId: tenantId, urlId: urlId, sso: sso) { (response, error) in
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