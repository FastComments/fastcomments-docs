Отпремите и промијените величину слике

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| sizePreset | string | query | Не | Подешавање величине: "Default" (1000x1000px) или "CrossPlatform" (креира величине за популарне уређаје) |
| urlId | string | query | Не | ИД странице са које се врши отпремање, за конфигурацију |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UploadImageResponse.swift)

## Пример

[inline-code-attrs-start title = 'uploadImage пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем, пријавите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let file = URL(string: "https://example.com")! // URL | 
let sizePreset = SizePreset() // SizePreset | Подешавање величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (креира величине за популарне уређаје) (опционо)
let urlId = "urlId_example" // String | ИД странице са које се врши отпремање, за конфигурацију (опционо)

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