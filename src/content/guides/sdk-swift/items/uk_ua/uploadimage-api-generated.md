Upload and resize an image

## Parameters

| Назва | Тип | Місце розташування | Обов'язково | Опис |
|------|------|--------------------|--------------|------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Попередньо заданий розмір: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв) |
| urlId | string | query | No | Ідентифікатор сторінки, з якої відбувається завантаження, для налаштування |

## Response

Повертає: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Example

[inline-code-attrs-start title = 'uploadImage Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще знаходяться у статусі бета. Якщо виникнуть проблеми, будь ласка, повідомте про них за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Попередньо заданий розмір: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв) (optional)
let urlId = "urlId_example" // String | Ідентифікатор сторінки, з якої відбувається завантаження, для налаштування (optional)

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