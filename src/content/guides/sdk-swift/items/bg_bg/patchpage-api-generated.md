## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Отговор

Връща: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchPageAPIResponse.swift)

## Пример

[inline-code-attrs-start title = 'patchPage Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни фрагменти от код все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateAPIPageData = UpdateAPIPageData(isClosed: false, accessibleByGroupIds: ["accessibleByGroupIds_example"], title: "title_example", url: "url_example", urlId: "urlId_example") // UpdateAPIPageData | 

DefaultAPI.patchPage(tenantId: tenantId, id: id, updateAPIPageData: updateAPIPageData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]