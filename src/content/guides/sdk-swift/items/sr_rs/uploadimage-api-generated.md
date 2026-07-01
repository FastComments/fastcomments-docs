Upload and resize an image

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
| tenantId | string | path | Да |  |
| sizePreset | string | query | Не | Претподешавање величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (прави величине за популарне уређаје) |
| urlId | string | query | Не | Идентификатор странице са које се врши отпремање, за конфигурацију |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Пример

[inline-code-attrs-start title = 'uploadImage Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било какав проблем, пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let file = URL(string: "https://example.com")! // URL |
let sizePreset = SizePreset() // SizePreset | Претподешавање величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (прави величине за популарне уређаје) (опционално)
let urlId = "urlId_example" // String | Идентификатор странице са које се врши отпремање, за конфигурацију (опционално)

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