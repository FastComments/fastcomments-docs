Otpremanje i promena veličine slike

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Size preset: \"Default\" (1000x1000px) ili \"CrossPlatform\" (kreira veličine za popularne uređaje) |
| urlId | string | query | No | ID stranice s koje se vrši upload, za konfiguraciju |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Example

[inline-code-attrs-start title = 'uploadImage Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći uzorci koda su još beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Size preset: \"Default\" (1000x1000px) ili \"CrossPlatform\" (kreira veličine za popularne uređaje) (optional)
let urlId = "urlId_example" // String | ID stranice s koje se vrši upload, za konfiguraciju (optional)

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