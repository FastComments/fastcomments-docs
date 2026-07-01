Upload og ændre størrelse på et billede

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Størrelsesforudindstilling: "Default" (1000x1000px) eller "CrossPlatform" (opretter størrelser for populære enheder) |
| urlId | string | query | No | side-id, som uploaden sker fra, for at konfigurere |

## Svar

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'uploadImage Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. Ved eventuelle problemer bedes du rapportere via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Størrelsesforudindstilling: \"Default\" (1000x1000px) eller \"CrossPlatform\" (opretter størrelser for populære enheder) (optional)
let urlId = "urlId_example" // String | side-id, som uploaden sker fra, for at konfigurere (optional)

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