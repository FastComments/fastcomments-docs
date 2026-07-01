Afbeelding uploaden en schalen

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Grootte‑voorinstelling: "Default" (1000x1000px) of "CrossPlatform" (maakt groottes voor populaire apparaten) |
| urlId | string | query | No | Pagina‑id vanwaar de upload plaatsvindt, om te configureren |

## Response

Retourneert: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'uploadImage Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Grootte‑voorinstelling: \"Default\" (1000x1000px) of \"CrossPlatform\" (maakt groottes voor populaire apparaten) (optioneel)
let urlId = "urlId_example" // String | Pagina‑id vanwaar de upload plaatsvindt, om te configureren (optioneel)

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