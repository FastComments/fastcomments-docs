## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tag | string | 路径 | 是 |  |
| tenantId | string | 查询 | 否 |  |

## 响应

返回: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## 示例

[inline-code-attrs-start title = 'deleteHashTag 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (可选)
let deleteHashTagRequest = DeleteHashTag_request(tenantId: "tenantId_example") // DeleteHashTagRequest |  (可选)

DefaultAPI.deleteHashTag(tag: tag, tenantId: tenantId, deleteHashTagRequest: deleteHashTagRequest) { (response, error) in
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