# Bir görüntüyü yükle ve yeniden boyutlandır

## Parameters

| Ad | Tür | Yer | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Boyut ön ayarı: "Default" (1000x1000px) veya "CrossPlatform" (popüler cihazlar için boyutlar oluşturur) |
| urlId | string | query | No | Yüklemenin gerçekleştiği sayfanın kimliği, yapılandırma için |

## Response

Döndürür: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Örnek

[inline-code-attrs-start title = 'uploadImage Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Boyut ön ayarı: \"Default\" (1000x1000px) veya \"CrossPlatform\" (popüler cihazlar için boyutlar oluşturur) (optional)
let urlId = "urlId_example" // String | Yüklemenin gerçekleştiği sayfanın kimliği, yapılandırma için (optional)

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