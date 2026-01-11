Otpremi i promijeni veličinu slike

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Preset veličine: "Default" (1000x1000px) or "CrossPlatform" (creates sizes for popular devices) |
| urlId | string | query | No | ID stranice sa koje se vrši upload, za konfiguraciju |

## Odgovor

Vraća: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Primjer

[inline-code-attrs-start title = 'uploadImage Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su još u beta fazi. Za bilo koji problem, prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Preset veličine: \"Default\" (1000x1000px) ili \"CrossPlatform\" (stvara veličine za popularne uređaje) (neobavezno)
let urlId = "urlId_example" // String | ID stranice sa koje se vrši upload, za konfiguraciju (neobavezno)

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