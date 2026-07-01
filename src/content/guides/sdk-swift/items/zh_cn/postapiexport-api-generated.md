## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| byIPFromComment | string | query | No |  |
| filters | string | query | No |  |
| searchFilters | string | query | No |  |
| sorts | string | query | No |  |
| sso | string | query | No |  |

## 响应

返回: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## 示例

[inline-code-attrs-start title = 'postApiExport 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 提交报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let textSearch = "textSearch_example" // String |  （可选）
let byIPFromComment = "byIPFromComment_example" // String |  （可选）
let filters = "filters_example" // String |  （可选）
let searchFilters = "searchFilters_example" // String |  （可选）
let sorts = "sorts_example" // String |  （可选）
let sso = "sso_example" // String |  （可选）

ModerationAPI.postApiExport(tenantId: tenantId, options: ModerationAPI.PostApiExportOptions(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]