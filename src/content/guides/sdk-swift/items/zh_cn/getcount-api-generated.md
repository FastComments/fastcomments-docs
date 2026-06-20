## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filter | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## 示例

[inline-code-attrs-start title = 'getCount 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let textSearch = "textSearch_example" // String | （可选）
let byIPFromComment = "byIPFromComment_example" // String | （可选）
let filter = "filter_example" // String | （可选）
let searchFilters = "searchFilters_example" // String | （可选）
let demo = true // Bool | （可选）
let sso = "sso_example" // String | （可选）

ModerationAPI.getCount(textSearch: textSearch, byIPFromComment: byIPFromComment, filter: filter, searchFilters: searchFilters, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]