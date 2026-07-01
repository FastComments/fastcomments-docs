Загрузка и изменение размера изображения

## Parameters

| Name      | Type   | Location | Required | Description |
|-----------|--------|----------|----------|-------------|
| tenantId  | string | path     | Yes      |  |
| sizePreset| string | query    | No       | Предустановки размеров: “Default” (1000x1000px) или “CrossPlatform” (создаёт размеры для популярных устройств) |
| urlId     | string | query    | No       | Идентификатор страницы, с которой происходит загрузка, для настройки (опционально) |

## Response

Возвращает: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте их по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Предустановки размеров: \"Default\" (1000x1000px) или \"CrossPlatform\" (создаёт размеры для популярных устройств) (опционально)
let urlId = "urlId_example" // String | Идентификатор страницы, с которой происходит загрузка, для настройки (опционально)

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