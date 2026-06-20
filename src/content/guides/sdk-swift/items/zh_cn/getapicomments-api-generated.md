## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| page | number | query | 否 |  |
| count | number | query | 否 |  |
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filters | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| sorts | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## 示例

[inline-code-attrs-start title = 'getApiComments 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let page = 987 // Double |  （可选）
let count = 987 // Double |  （可选）
let textSearch = "textSearch_example" // String |  （可选）
let byIPFromComment = "byIPFromComment_example" // String |  （可选）
let filters = "filters_example" // String |  （可选）
let searchFilters = "searchFilters_example" // String |  （可选）
let sorts = "sorts_example" // String |  （可选）
let demo = true // Bool |  （可选）
let sso = "sso_example" // String |  （可选）

ModerationAPI.getApiComments(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso) { (response, error) in
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