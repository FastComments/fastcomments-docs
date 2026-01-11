Upload og ændre størrelse på et billede

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nej | Størrelsesforindstilling: "Default" (1000x1000px) eller "CrossPlatform" (opretter størrelser til populære enheder) |
| urlId | string | query | Nej | Side-id hvorfra upload foregår, til konfiguration |

## Svar

Returnerer: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'uploadImage Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Størrelsesforindstilling: \"Default\" (1000x1000px) eller \"CrossPlatform\" (opretter størrelser til populære enheder) (valgfrit)
let urlId = "urlId_example" // String | Side-id hvorfra upload foregår, til konfiguration (valgfrit)

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