Завантажити та змінити розмір зображення

## Parameters

| Назва | Тип | Location | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| sizePreset | string | query | Ні | Пресет розміру: "Default" (1000x1000px) або "CrossPlatform" (створює розміри для популярних пристроїв) |
| urlId | string | query | Ні | Ідентифікатор сторінки, звідки відбувається завантаження, для конфігурації |

## Response

Повертає: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад uploadImage'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще в бета-версії. У разі проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Пресет розміру: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв) (optional)
let urlId = "urlId_example" // String | Ідентифікатор сторінки, звідки відбувається завантаження, для конфігурації (optional)

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