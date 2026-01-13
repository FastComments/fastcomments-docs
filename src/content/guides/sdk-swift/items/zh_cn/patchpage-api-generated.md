## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## 响应

返回: [`PatchPageAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PatchPageAPIResponse.swift)

## 示例

[inline-code-attrs-start title = 'patchPage 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如果有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
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