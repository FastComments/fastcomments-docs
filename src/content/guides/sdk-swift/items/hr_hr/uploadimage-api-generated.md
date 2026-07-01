Prenesi i promijeni veličinu slike

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Predložak veličine: "Default" (1000x1000px) ili "CrossPlatform" (stvara veličine za popularne uređaje) |
| urlId | string | query | No | ID stranice s koje se vrši prijenos, za konfiguraciju |

## Odgovor

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda još su beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Predložak veličine: \"Default\" (1000x1000px) ili \"CrossPlatform\" (stvara veličine za popularne uređaje) (optional)
let urlId = "urlId_example" // String | ID stranice s koje se vrši prijenos, za konfiguraciju (optional)

PublicAPI.uploadImage(tenantId: tenantId, file: file, options: PublicAPI.UploadImageOptions(sizePreset: sizePreset, urlId: urlId)) { (response, error) in
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