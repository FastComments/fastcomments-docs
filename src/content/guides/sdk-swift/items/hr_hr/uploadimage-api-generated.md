Učitajte i promijenite veličinu slike

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| sizePreset | string | query | Ne | Prednastavka veličine: \"Default\" (1000x1000px) ili \"CrossPlatform\" (stvara veličine za popularne uređaje) |
| urlId | string | query | Ne | ID stranice s koje se vrši prijenos, za konfiguraciju |

## Odgovor

Vraća: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda još su u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Prednastavka veličine: \"Default\" (1000x1000px) ili \"CrossPlatform\" (stvara veličine za popularne uređaje) (optional)
let urlId = "urlId_example" // String | ID stranice s koje se vrši prijenos, za konfiguraciju (neobavezno)

PublicAPI.uploadImage(tenantId: tenantId, file: file, sizePreset: sizePreset, urlId: urlId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]