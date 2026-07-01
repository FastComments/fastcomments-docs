Upload and resize an image → Prześlij i zmień rozmiar obrazu

## Parameters → Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Predefiniowany rozmiar: \"Default\" (1000x1000px) lub \"CrossPlatform\" (tworzy rozmiary dla popularnych urządzeń) |
| urlId | string | query | No | Id strony, z której odbywa się przesyłanie, do konfiguracji |

## Response → Odpowiedź

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift) → Zwraca:

## Example → Przykład

[inline-code-attrs-start title = 'uploadImage Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są wciąż w wersji beta. W razie problemów zgłoś je pod http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Predefiniowany rozmiar: \"Default\" (1000x1000px) lub \"CrossPlatform\" (tworzy rozmiary dla popularnych urządzeń) (optional)
let urlId = "urlId_example" // String | Id strony, z której odbywa się przesyłanie, do konfiguracji (optional)

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