---
Загрузить и изменить размер изображения

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Предустановленный размер: "Default" (1000x1000px) или "CrossPlatform" (создаёт размеры для популярных устройств) |
| urlId | string | query | No | Идентификатор страницы, из которой происходит загрузка, для настройки |

## Ответ

Возвращает: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода находятся в бета-версии. При возникновении проблем, пожалуйста, сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Предустановленный размер: \"Default\" (1000x1000px) или \"CrossPlatform\" (создаёт размеры для популярных устройств) (необязательно)
let urlId = "urlId_example" // String | Идентификатор страницы, из которой происходит загрузка, для настройки (необязательно)

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

---