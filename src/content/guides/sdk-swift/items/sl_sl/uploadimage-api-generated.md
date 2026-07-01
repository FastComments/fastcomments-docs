Naloži in spremeni velikost slike

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| sizePreset | string | query | Ne | Velikostni prednastavitev: "Default" (1000x1000px) ali "CrossPlatform" (ustvari velikosti za priljubljene naprave) |
| urlId | string | query | Ne | ID strani, s katere se izvaja nalaganje, za konfiguracijo |

## Odgovor

Vrne: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Primer

[inline-code-attrs-start title = 'uploadImage Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta različici. Za kakršnekoli težave, prosimo, poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Velikostni prednastavitev: \"Default\" (1000x1000px) ali \"CrossPlatform\" (ustvari velikosti za priljubljene naprave) (neobvezno)
let urlId = "urlId_example" // String | ID strani, s katere se izvaja nalaganje, za konfiguracijo (neobvezno)

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