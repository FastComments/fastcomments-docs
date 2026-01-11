Bild hochladen und skalieren

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Größenvoreinstellung: "Default" (1000x1000px) oder "CrossPlatform" (erstellt Größen für gängige Geräte) |
| urlId | string | query | No | Seiten-ID, von der der Upload erfolgt, zur Konfiguration |

## Antwort

Gibt zurück: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'uploadImage Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte über http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Größenvoreinstellung: \"Default\" (1000x1000px) oder \"CrossPlatform\" (erstellt Größen für gängige Geräte) (optional)
let urlId = "urlId_example" // String | Seiten-ID, von der der Upload erfolgt, zur Konfiguration (optional)

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