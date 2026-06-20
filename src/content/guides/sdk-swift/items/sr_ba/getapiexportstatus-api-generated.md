## Параметри

| Име | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| batchJobId | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportStatusResponse.swift)

## Пример

[inline-code-attrs-start title = 'getApiExportStatus Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још увек у бета фази. За било који проблем, пријавите преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let batchJobId = "batchJobId_example" // String |  (опционо)
let sso = "sso_example" // String |  (опционо)

ModerationAPI.getApiExportStatus(batchJobId: batchJobId, sso: sso) { (response, error) in
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