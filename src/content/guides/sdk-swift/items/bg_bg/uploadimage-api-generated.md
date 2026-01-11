Качване и преоразмеряване на изображение

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| sizePreset | string | query | Не | Предварително зададен размер: "Default" (1000x1000px) или "CrossPlatform" (създава размери за популярни устройства) |
| urlId | string | query | Не | Идентификатор на страницата, от която се извършва качването, за конфигуриране |

## Отговор

Връща: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са в бета версия. За проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // Size preset: \"Default\" (1000x1000px) or \"CrossPlatform\" (създава размери за популярни устройства) (по избор)
let urlId = "urlId_example" // String | Идентификатор на страницата, от която се извършва качването, за конфигуриране (по избор)

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