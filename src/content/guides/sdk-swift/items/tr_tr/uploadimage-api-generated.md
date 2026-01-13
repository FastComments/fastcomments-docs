Resim yükleme ve yeniden boyutlandırma

## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| sizePreset | string | query | Hayır | Boyut ön ayarı: "Default" (1000x1000px) veya "CrossPlatform" (popüler cihazlar için boyutlar oluşturur) |
| urlId | string | query | Hayır | Yüklemenin gerçekleştiği sayfanın kimliği, yapılandırma için |

## Yanıt

Döndürür: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Örnek

[inline-code-attrs-start title = 'uploadImage Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Boyut ön ayarı: \"Default\" (1000x1000px) veya \"CrossPlatform\" (popüler cihazlar için boyutlar oluşturur) (isteğe bağlı)
let urlId = "urlId_example" // String | Yüklemenin gerçekleştiği sayfanın kimliği, yapılandırma için (isteğe bağlı)

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