Отпремање и промена величине слике

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Подешавање величине: "Default" (1000x1000px) или "CrossPlatform" (креира величине за популарне уређаје) |
| urlId | string | query | No | Ид странице са које се врши отпремање, за конфигурисање |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Пример

[inline-code-attrs-start title = 'uploadImage Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета верзији. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Подешавање величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (креира величине за популарне уређаје) (опционо)
let urlId = "urlId_example" // String | Ид странице са које се врши отпремање, за конфигурисање (опционо)

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