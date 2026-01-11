Otpremi i promeni veličinu slike

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| sizePreset | string | query | Ne | Predefinisana veličina: "Default" (1000x1000px) ili "CrossPlatform" (kreira veličine za popularne uređaje) |
| urlId | string | query | Ne | ID stranice sa koje se vrši otpremanje, za konfiguraciju |

## Odgovor

Vraća: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Primer

[inline-code-attrs-start title = 'uploadImage Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Predefinisana veličina: \"Default\" (1000x1000px) ili \"CrossPlatform\" (kreira veličine za popularne uređaje) (opciono)
let urlId = "urlId_example" // String | ID stranice sa koje se vrši otpremanje, za konfiguraciju (opciono)

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

---