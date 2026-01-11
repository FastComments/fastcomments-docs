Загрузить и изменить размер изображения

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Предустановка размера: "Default" (1000x1000px) или "CrossPlatform" (создаёт размеры для популярных устройств) |
| urlId | string | query | No | Идентификатор страницы, с которой выполняется загрузка, для конфигурации |

## Ответ

Возвращает: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Пример

[inline-code-attrs-start title = 'uploadImage Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. Для сообщений об ошибках, пожалуйста, создайте issue по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Предустановка размера: \"Default\" (1000x1000px) или \"CrossPlatform\" (создаёт размеры для популярных устройств) (необязательно)
let urlId = "urlId_example" // String | Идентификатор страницы, с которой выполняется загрузка, для конфигурации (необязательно)

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