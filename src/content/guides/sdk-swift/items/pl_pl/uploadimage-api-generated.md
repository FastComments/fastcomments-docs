Prześlij i zmień rozmiar obrazu

## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| sizePreset | string | query | Nie | Predefiniowany rozmiar: "Default" (1000x1000px) lub "CrossPlatform" (tworzy rozmiary dla popularnych urządzeń) |
| urlId | string | query | Nie | Id strony, z której odbywa się przesyłanie, do konfiguracji |

## Odpowiedź

Zwraca: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Predefiniowany rozmiar: \"Default\" (1000x1000px) lub \"CrossPlatform\" (tworzy rozmiary dla popularnych urządzeń) (opcjonalne)
let urlId = "urlId_example" // String | Id strony, z której odbywa się przesyłanie, do konfiguracji (opcjonalne)

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