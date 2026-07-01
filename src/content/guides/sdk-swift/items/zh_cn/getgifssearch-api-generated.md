## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| search | string | 查询 | 是 |  |
| locale | string | 查询 | 否 |  |
| rating | string | 查询 | 否 |  |
| page | number | 查询 | 否 |  |

## 响应

返回：[`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetGifsSearchResponse.swift)

## 示例

[inline-code-attrs-start title = 'getGifsSearch 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let search = "search_example" // String | 
let locale = "locale_example" // String |  (可选)
let rating = "rating_example" // String |  (可选)
let page = 987 // Double |  (可选)

PublicAPI.getGifsSearch(tenantId: tenantId, search: search, options: PublicAPI.GetGifsSearchOptions(locale: locale, rating: rating, page: page)) { (response, error) in
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