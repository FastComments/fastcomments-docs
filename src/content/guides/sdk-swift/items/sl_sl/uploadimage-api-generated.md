Naloži in spremeni velikost slike

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Prednastavitev velikosti: "Default" (1000x1000px) ali "CrossPlatform" (ustvari velikosti za priljubljene naprave) |
| urlId | string | query | No | ID strani, iz katere poteka nalaganje, za konfiguracijo |

## Odgovor

Vrača: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Primer

[inline-code-attrs-start title = 'uploadImage Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za kakršen koli problem, prosimo prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Prednastavitev velikosti: \"Default\" (1000x1000px) ali \"CrossPlatform\" (ustvari velikosti za priljubljene naprave) (optional)
let urlId = "urlId_example" // String | ID strani, iz katere poteka nalaganje, za konfiguracijo (optional)

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